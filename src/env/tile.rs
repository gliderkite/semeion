use std::cell::RefMut;

use super::*;

/// A 1-dimensional list of tiles that represents a grid of given bounds with
/// squared tiles of with the same side length.
/// Only entities that have a defined location will be stored in this data
/// structure.
#[derive(Debug)]
pub struct Tiles<'e, I: Eq + Hash, K, C, T, E> {
    tiles: Vec<Tile<'e, I, K, C, T, E>>,
    bounds: Bounds,
}

impl<'e, I: Eq + Hash + Clone + Debug, K, C, T, E> Tiles<'e, I, K, C, T, E> {
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
    pub fn insert(
        &mut self,
        entity: &EntityStrongRef<'e, I, K, C, T, E>,
    ) -> bool {
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
        entity: &RefMut<entity::Trait<'e, I, K, C, T, E>>,
    ) -> Option<NeighborHood<'_, 'e, I, K, C, T, E>> {
        match (entity.location(), entity.scope()) {
            // only entities that have both a scope and a location can interact
            // with the surrounding environment
            (Some(center), Some(scope)) => {
                let id = entity.id();
                let mut tiles =
                    Vec::with_capacity(Bounds::len_with_scope(scope));

                let scope = scope.magnitude() as i32;
                // build the portion of the environment seen by the entity tile
                // by tile from the top-left corner to the bottom-down corner
                for y in -scope..=scope {
                    for x in -scope..=scope {
                        let index = center
                            .clone()
                            .translate(Offset { x, y }, self.bounds)
                            .one_dimensional(self.bounds);
                        let tile = &self.tiles[index];
                        tiles.push(TileView {
                            entity_id: id.clone(),
                            tile,
                        });
                    }
                }

                Some(tiles.into())
            }
            _ => None,
        }
    }
}

/// A single tile of the environment. This data structure contains a map of weak
/// references to the entities.
#[derive(Debug)]
pub struct Tile<'e, I: Eq + Hash, K, C, T, E> {
    entities: HashMap<I, EntityWeakRef<'e, I, K, C, T, E>>,
}

impl<'e, I: Eq + Hash, K, C, T, E> Default for Tile<'e, I, K, C, T, E> {
    /// Constructs an empty Tile.
    fn default() -> Self {
        Self {
            entities: HashMap::default(),
        }
    }
}

/// A single Tile as seen from a specific entity when it belongs to a NeighborHood.
#[derive(Debug)]
pub struct TileView<'a, 'e, I: Eq + Hash + Debug, K, C, T, E> {
    // the ID of the Entity that is seeing this tile
    entity_id: I,
    // the reference to the Tile in the Environment, where the weak references
    // to the entities are stored
    tile: &'a Tile<'e, I, K, C, T, E>,
}

impl<'a, 'e, I: Eq + Hash + Clone + Debug, K, C, T, E>
    TileView<'a, 'e, I, K, C, T, E>
{
    /// Gets a list of the entities located in this Tile that does not contain
    /// the entity who is "owning" the tile (and prevents from borrowing the same
    /// entity twice).
    pub fn entities(&self) -> Vec<EntityStrongRef<'e, I, K, C, T, E>> {
        self.tile
            .entities
            .iter()
            .filter_map(move |(id, e)| {
                if &self.entity_id != id {
                    e.upgrade()
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets the total number of entities located in this Tile, including the
    /// entity that "owns" the tile.
    pub fn len(&self) -> usize {
        self.tile.entities.len()
    }

    /// Returns true only if there are no entities located in this tile.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
