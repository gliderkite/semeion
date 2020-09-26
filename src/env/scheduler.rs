//! This module contains the Scheduler core logic, responsible of assigning each
//! Entity of the Environment to its own task.
//! The correctness of the unsafe blocks used in the engine code mostly depends
//! on a sound implementation of the Scheduler logic, therefore this represents
//! a main core component of the whole library.
//!
//! The idea is to exploit the geometry of the Environment, that represents a
//! 2-dimensional grid of tiles, and therefore, can be split into multiple areas.
//! Each of these areas (or tiles that can contain multiple Environment tiles),
//! will include multiple entities, and all the operations performed are at least
//! theoretically parallelizable.
//! Particular attention must be payed to the synchronization between entities,
//! so that no mutable references to any entity is shared (no aliasing). This is
//! especially important when taking into consideration that each Entity can query
//! its neighbors according to its Scope, and beside avoiding that mutable
//! references to a neighbor Entity is shared at the same time, we need to ensure
//! that there will be no situations where two different neighbor entities query
//! each own surroundings resulting into a deadlock.
//!
//! The solution to this problem consist in splitting the Environment into (N)
//! multiple sub tiles (usually as many as the number of cores), to the assign
//! each Entity according to its location to one of these tiles; unless the
//! visible neighborhood for the Entity (that depends on the magnitude of its
//! Scope) intersects a different tile, if that is the case the Entity is going
//! to be assigned to a special tile (N + 1).
//! The first N tiles are those tiles which entities require to be synchronized
//! only between each other, that is synchronization is required only between
//! entities belonging to the same tile (and this is achieved by running the
//! entities operation belonging to the same tile on the same thread - but the N
//! subset of entities can be run in parallel).
//! The special N + 1 tile requires a more strict synchronization, since each of
//! its entities requires (excluding further possible optimizations) to be
//! synchronized with any other entity, and therefore, these entities operations
//! must be run on the same thread only after all the N previous tiles entities
//! operations are completed.

use std::collections::BTreeMap;

use super::*;

unsafe impl<'e, K, C> Send for Tiles<'e, K, C> {}
unsafe impl<'e, K, C> Sync for Tiles<'e, K, C> {}

/// The multithreaded scheduler in charge of correctly dispatching events to all
/// the entities in the environment.
#[derive(Debug)]
pub struct Scheduler {
    grid: Grid,
    jobs: usize,
}

/// This data structure contains a list of entities separated according to the
/// ones that can be processed in parallel from the ones that require to be
/// synchronized and can only be processed after all the others.
pub struct Tasks<'a, 'e, K, C> {
    // The list of entities that have been split into multiple subsets, each of
    // which can be run on parallel. These entities do not need to be synchronized
    // between different set (but still need to be synchronized if part of the
    // same set).
    pub sync: Vec<Vec<&'a mut entity::Trait<'e, K, C>>>,
    // The list of entities that cannot be processed in parallel, and that need
    // to wait until all the sync entities have been processed first. These
    // entities need to be synchronized between each other and also with all the
    // other entities belonging to the sync sets.
    pub unsync: Vec<&'a mut entity::Trait<'e, K, C>>,
}

impl Scheduler {
    /// Constructs a new Scheduler for an environment with the given dimension
    /// and the number of parallel jobs that will be used by it.
    pub fn new(dimension: impl Into<Dimension>, jobs: usize) -> Self {
        debug_assert!(jobs > 0);
        Self {
            grid: Grid::new(dimension, jobs),
            jobs,
        }
    }

    /// Given a list of entities, separates them into a list of Tasks that can
    /// be either run on parallel or require strict synchronization with all the
    /// other entities.
    pub fn get_tasks<'a, 'e, K, C>(
        &self,
        entities: impl IntoIterator<Item = &'a mut entity::Trait<'e, K, C>>,
    ) -> Tasks<'a, 'e, K, C> {
        debug_assert!(self.jobs > 0);
        if self.jobs == 1 {
            return Tasks {
                sync: Vec::default(),
                unsync: entities.into_iter().collect(),
            };
        }

        // list of entities that do not require synchronization between different
        // sets of entities of this list
        let mut sync = Vec::new();
        sync.resize_with(
            self.grid.dimension.len(),
            Vec::<&mut entity::Trait<'e, K, C>>::default,
        );
        // list of entities that require synchronization with all the other entities
        let mut unsync = Vec::new();

        // assign each entity to its own task
        for e in entities {
            if let Some(location) = e.location() {
                let scope = e.scope().unwrap_or_else(Scope::empty);
                // each entity must be assigned to its own tile, if the tile
                // cannot be found it's an unrecoverable internal error
                let tile =
                    self.grid.get(location, scope).unwrap_or_else(|| {
                        panic!(
                            "Cannot assign Tile to Entity at {:?} with {:?}",
                            location, scope
                        )
                    });

                match tile {
                    Tile::Sync { index } => sync[index].push(e),
                    Tile::Unsync => unsync.push(e),
                };
            } else {
                // if an Entity has no location the task to which it can be
                // assigned is arbitrary
                unsync.push(e);
            }
        }

        Tasks { sync, unsync }
    }
}

/// The coordinate in space of a 2-dimensional Location (Point), that could
/// either represents its abscissa or ordinate.
type Coordinate = i32;

/// The ID of a rectangular tile, subset of the main rectangular area that
/// represents the whole Environment. Each tile has an unique ID that
/// represents its index in the list of Environment tiles if these were to be
/// aligned one after the other (instead of being displayed as part of a
/// 2-dimensional grid).
type TileIndex = usize;

/// The map of tiles edges, where the key is the edge coordinate, and the value
/// is the ID of the tile. These could represent either the horizontal edges or
/// the vertical edges.
type EdgesMap = BTreeMap<Coordinate, TileIndex>;

/// The Tile of the Environment as seen from the Scheduler. Each of these Tiles
/// usually cover a portion of the Environment that includes multiple entities
/// (and multiple env::tile::Tile), and its dimensions depends on the number of
/// jobs used to split the Grid that the Scheduler will use to separate the
/// various entities into multiple parallel tasks. If of these Tile can be
/// identified by an index, unless it represents a special Tile that will include
/// all the entities that require special synchronization.
#[derive(Debug)]
enum Tile {
    Sync { index: TileIndex },
    Unsync,
}

/// A rectangular grid of equal sized tiles.
#[derive(Debug)]
struct Grid {
    dimension: Dimension,
    vertical: EdgesMap,
    horizontal: EdgesMap,
}

impl Grid {
    /// Given a rectangle with the given Dimension, splits the rectangle into
    /// `count` smaller equally sized rectangles, and constructs a new Grid made
    /// from these rectangles.
    ///
    /// For example, given a dimension (A) of 4x4 and a count equal to 4, the
    /// mapped dimension (B) of the new Grid would be equal to 2x2, where each
    /// tile of the new Grid, would have to have dimension of 2x2 in order to
    /// fill up the dimension (A).
    fn new(dimension: impl Into<Dimension>, count: usize) -> Self {
        let grid_dimension = Dimension::with_eq_rectangles(count);
        debug_assert!(!grid_dimension.is_empty());

        let dimension = dimension.into();
        let tile_dimension = grid_dimension.scale(dimension);

        let step = tile_dimension.x as usize;
        let count = grid_dimension.x as usize + 1;
        // the first vertical edge starts at the origin (left)
        let vertical: EdgesMap = (0..)
            .step_by(step)
            .take(count)
            .enumerate()
            // and the last edge ends with an abscissa equal to the given dimension
            .map(|(i, x)| (if i == count - 1 { dimension.x } else { x }, i))
            .collect();

        let step = tile_dimension.y as usize;
        let count = grid_dimension.y as usize + 1;
        // the first horizontal edge starts at the origin (top)
        let horizontal: EdgesMap = (0..=dimension.y)
            .step_by(step)
            .take(count)
            .enumerate()
            // and the last edge ends with an ordinate equal to the given dimension
            .map(|(i, y)| (if i == count - 1 { dimension.y } else { y }, i))
            .collect();

        Self {
            dimension: grid_dimension,
            vertical,
            horizontal,
        }
    }

    /// Gets the Tile of this Grid that contains the given Location.
    fn get(&self, location: Location, scope: Scope) -> Option<Tile> {
        // Returns a tuple where the first element is the index of the tile that
        // contains the given coordinate, and the second element is a tuple
        // with the 2 coordinates of the sides of the tile.
        let tile_edges = |edges: &EdgesMap, coordinate| {
            use std::ops::Bound::*;

            // get the lower bound coordinate and tile index
            let mut range = edges.range((Unbounded, Included(coordinate)));
            let (&low_bound, &index) = range.next_back()?;
            // get the upper bound coordinate
            let mut range = edges.range((Excluded(coordinate), Unbounded));
            let (&up_bound, _) = range.next()?;
            Some((index as i32, (low_bound, up_bound)))
        };

        let (x, (left, right)) = tile_edges(&self.vertical, location.x)?;
        let (y, (top, bottom)) = tile_edges(&self.horizontal, location.y)?;
        let index = Location { x, y }.one_dimensional(self.dimension);

        let scope = scope.magnitude() as i32;
        if left > location.x - scope || right < location.x + scope {
            // the scope goes beyond the tile that contains the given location
            return Some(Tile::Unsync);
        }
        if top > location.y - scope || bottom < location.y + scope {
            // the scope goes beyond the tile that contains the given location
            return Some(Tile::Unsync);
        }

        Some(Tile::Sync { index })
    }
}
