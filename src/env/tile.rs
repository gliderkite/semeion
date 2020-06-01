use std::cell::RefMut;

use super::*;

/// A 1-dimensional list of tiles that represents a grid of given bounds with
/// squared tiles of with the same side length.
/// Only entities that have a defined location will be stored in this data
/// structure.
#[derive(Debug)]
pub struct Tiles<I: Eq + Hash, K, C, T, E> {
    tiles: Vec<Tile<I, K, C, T, E>>,
    bounds: Bounds,
}

impl<I: Eq + Hash + Clone + Debug, K, C, T, E> Tiles<I, K, C, T, E> {
    /// Constructs a new list of tiles of the given bounds with no entities
    /// assigned to it.
    pub fn new(bounds: Bounds) -> Self {
        let mut tiles = Vec::with_capacity(bounds.len());
        tiles.resize_with(tiles.capacity(), Tile::default);
        Self { tiles, bounds }
    }

    /// Inserts a weak reference to the entity in the grid according to its
    /// location. If the entity has not location it will not be inserted.
    /// Returns whether the entity was inserted or not.
    pub fn insert(&mut self, entity: &EntityStrongRef<I, K, C, T, E>) -> bool {
        let location = entity.borrow().location();
        if let Some(location) = location {
            let index = location.one_dimensional(self.bounds);
            let tile = &mut self.tiles[index];
            let w = Rc::downgrade(entity);
            tile.entities.insert(entity.borrow().id().clone(), w);
            true
        } else {
            false
        }
    }

    /// Remove the entity with the given ID from the given location. Returns
    /// whether the entity was removed or not.
    pub fn remove(&mut self, id: &I, location: Location) -> bool {
        let index = location.one_dimensional(self.bounds);
        let tile = &mut self.tiles[index];
        tile.entities.remove(id).is_some()
    }

    /// Move the entity with the given ID between a previous and a new location.
    pub fn swap(&mut self, id: &I, from: Location, to: Location) {
        let index = from.one_dimensional(self.bounds);
        let tile = &mut self.tiles[index];

        if let Some(e) = tile.entities.remove(&id) {
            let index = to.one_dimensional(self.bounds);
            let tile = &mut self.tiles[index];
            tile.entities.insert(id.clone(), e);
        }
    }

    /// Gets the area of the environment surrounding the given location.
    pub fn neighborhood(
        &self,
        entity: &RefMut<entity::Trait<I, K, C, T, E>>,
    ) -> Option<NeighborHood<I, K, C, T, E>> {
        match (entity.location(), entity.scope()) {
            // only entities that have both a scope and a location can interact
            // with the surrounding environment
            (Some(center), Some(scope)) => {
                let id = entity.id();
                let mut tiles = Vec::with_capacity(Self::tiles_count(scope));
                let side = Self::tiles_side_len(scope) as i32;
                let scope = scope as i32;
                // build the portion of the environment seen by the entity tile
                // by tile from the top-left corner to the bottom-down corner
                for y in -scope..=scope {
                    for x in -scope..=scope {
                        let index = center
                            .clone()
                            .translate(Location { x, y }, self.bounds)
                            .one_dimensional(self.bounds);
                        let tile = &self.tiles[index];
                        tiles.push(TileView {
                            id: id.clone(),
                            tile,
                        });
                    }
                }
                Some(NeighborHood {
                    tiles,
                    bounds: Bounds { x: side, y: side },
                })
            }
            _ => None,
        }
    }

    /// Gets the number or rows (and columns) or a grid with equal number of
    /// rows and columns according to the given scope.
    fn tiles_side_len(scope: usize) -> usize {
        1 + scope.saturating_sub(1) * 2
    }

    /// Given a scope returns the number of tiles that are included in the
    /// portion of the environment within that scope.
    fn tiles_count(scope: usize) -> usize {
        match scope {
            0 => 1,
            _ => {
                let side = Self::tiles_side_len(scope);
                Self::tiles_count(scope - 1) + (side * 4 + 4)
            }
        }
    }
}

/// A single tile of the environment. This data structure contains a map of weak
/// references to the entities.
#[derive(Debug)]
pub struct Tile<I: Eq + Hash, K, C, T, E> {
    entities: HashMap<I, EntityWeakRef<I, K, C, T, E>>,
}

impl<I: Eq + Hash, K, C, T, E> Default for Tile<I, K, C, T, E> {
    /// Constructs an empty Tile.
    fn default() -> Self {
        Self {
            entities: HashMap::default(),
        }
    }
}

/// A single Tile as seen from a specific entity.
pub struct TileView<'a, I: Eq + Hash + Debug, K, C, T, E> {
    tile: &'a Tile<I, K, C, T, E>,
    id: I,
}

impl<'a, I: Eq + Hash + Clone + Debug, K, C, T, E> TileView<'a, I, K, C, T, E> {
    /// Gets a list of the entities located in this Tile that does not contain
    /// the entity who is "owning" the tile (and prevents from borrowing the same
    /// entity twice).
    pub fn entities(&self) -> Vec<EntityStrongRef<I, K, C, T, E>> {
        self.tile
            .entities
            .iter()
            .filter_map(
                move |(i, e)| if &self.id != i { e.upgrade() } else { None },
            )
            .collect()
    }
}
