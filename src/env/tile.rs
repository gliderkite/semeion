use super::*;

/// A 1-dimensional list of tiles that represents a grid of given dimension with
/// squared tiles of the same side length.
/// Only entities that have a defined location will be stored in this data
/// structure.
#[derive(Debug)]
pub struct Tiles<'e, I, K, C, T, E> {
    dimension: Dimension,
    tiles: Vec<Tile<'e, I, K, C, T, E>>,
}

impl<'e, I: Eq + Hash + Clone, K, C, T, E> Tiles<'e, I, K, C, T, E> {
    /// Constructs a new list of tiles of the given dimension with no entities
    /// assigned to it.
    pub fn new(dimension: Dimension) -> Self {
        let mut tiles = Vec::new();
        tiles.resize_with(dimension.len(), Tile::default);
        Self { tiles, dimension }
    }

    /// Inserts the given Entity in the grid according to its location. If the
    /// Entity has not location it will not be inserted.
    /// Returns whether the Entity was inserted or not.
    pub fn insert(
        &mut self,
        entity: &mut entity::Trait<'e, I, K, C, T, E>,
    ) -> bool {
        if let Some(location) = entity.location() {
            let index = location.one_dimensional(self.dimension);
            debug_assert!(index < self.tiles.len());
            let tile = &mut self.tiles[index];
            tile.entities.insert(
                entity.id().clone(),
                entity as *mut entity::Trait<'e, I, K, C, T, E>,
            );
            true
        } else {
            false
        }
    }

    /// Remove the Entity with the given ID from the given location.
    /// Returns whether the Entity was removed or not.
    pub fn remove(&mut self, id: &I, location: Location) -> bool {
        let index = location.one_dimensional(self.dimension);
        debug_assert!(index < self.tiles.len());
        let tile = &mut self.tiles[index];
        tile.entities.remove(id).is_some()
    }

    /// Move the Entity with the given ID between a previous and a new location.
    pub fn relocate(&mut self, id: &I, from: Location, to: Location) {
        let index = from.one_dimensional(self.dimension);
        debug_assert!(index < self.tiles.len());
        let tile = &mut self.tiles[index];

        if let Some(e) = tile.entities.remove(&id) {
            let index = to.one_dimensional(self.dimension);
            let tile = &mut self.tiles[index];
            tile.entities.insert(id.clone(), e);
        }
    }

    /// Gets an iterator over all the entities located at the given location.
    /// The Environment is seen as a Torus from this method, therefore, out of
    /// bounds offsets will be translated considering that the Environment
    /// edges are joined.
    pub fn entities_at(
        &self,
        location: Location,
    ) -> impl Iterator<Item = &entity::Trait<'e, I, K, C, T, E>> {
        let index = location.one_dimensional(self.dimension);
        debug_assert!(index < self.tiles.len());
        let tile = &self.tiles[index];
        tile.entities()
    }

    /// Gets the area of the environment surrounding the given Entity.
    /// Returns None if the Entity has no location or scope, or if the scope of
    /// the Entity forces its neighborhood to wrap onto itself due to the
    /// dimensions of the Environment being not big enough to contain it.
    pub fn neighborhood(
        &mut self,
        entity: &entity::Trait<'e, I, K, C, T, E>,
    ) -> Option<NeighborHood<'_, 'e, I, K, C, T, E>> {
        match (entity.location(), entity.scope()) {
            // only entities that have both a scope and a location can interact
            // with the surrounding environment
            (Some(center), Some(scope)) => {
                if scope.overflows(self.dimension) {
                    // the dimension of the environment are not big enough to
                    // construct a valid neighborhood given this entity scope
                    return None;
                }

                let mut neighborhood =
                    Vec::with_capacity(Dimension::len_with_scope(scope));
                let scope = scope.magnitude() as i32;
                let tiles_slice = self.tiles.as_mut_ptr();

                // build the portion of the environment seen by the entity tile
                // by tile from the top-left corner to the bottom-down corner
                for y in -scope..=scope {
                    for x in -scope..=scope {
                        let index = center
                            .clone()
                            .translate(Offset { x, y }, self.dimension)
                            .one_dimensional(self.dimension);
                        debug_assert!(index < self.tiles.len());
                        neighborhood.push(TileView::with_owner(
                            entity.id().clone(),
                            // This is safe only if the indexes are all unique,
                            // which is guaranteed by (1) an Entity's neighbors
                            // cells cannot wrap onto itself even in a small
                            // torus Environment, (2) the (x, y) coordinates
                            // yielded by this loop are unique, and (3) the index
                            // is within the tiles vector bounds.
                            // Having unique indexes guarantees to not have
                            // multiple mutable references to the same Tile, even
                            // when this cannot be checked by the type system.
                            unsafe { &mut *tiles_slice.add(index) },
                        ));
                    }
                }

                Some(neighborhood.into())
            }
            _ => None,
        }
    }
}

/// A single tile of the environment. This data structure contains a map of
/// *weak* references to the entities.
#[derive(Debug)]
pub struct Tile<'e, I, K, C, T, E> {
    entities: HashMap<I, *mut entity::Trait<'e, I, K, C, T, E>>,
}

impl<'e, I, K, C, T, E> Default for Tile<'e, I, K, C, T, E> {
    /// Constructs an empty Tile.
    fn default() -> Self {
        Self {
            entities: HashMap::default(),
        }
    }
}

impl<'e, I: Eq + Clone, K, C, T, E> Tile<'e, I, K, C, T, E> {
    /// Gets an iterator over all the entities located in this Tile.
    /// The entities are returned in arbitrary order.
    pub fn entities(
        &self,
    ) -> impl Iterator<Item = &entity::Trait<'e, I, K, C, T, E>> {
        self.entities.iter().filter_map(move |(_id, e)| {
            // Dereferencing the Entity pointer to return its reference
            // is safe because the Environment guarantees that this
            // method can only be called while the Entity pointed by this
            // pointer still exist, and its lifetime will be equal to
            // greater than the lifetime of self, besides the fact that is
            // was properly allocated and initialized.
            unsafe { e.as_ref() }
        })
    }

    /// Gets an iterator over all the mutable entities located in this Tile.
    /// The entities are returned in arbitrary order.
    pub fn entities_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut entity::Trait<'e, I, K, C, T, E>> {
        self.entities.iter_mut().filter_map(move |(_id, e)| {
            // Dereferencing the Entity pointer to return its reference
            // is safe because the Environment guarantees that this
            // method can only be called while the Entity pointed by this
            // pointer still exist, and its lifetime will be equal to
            // greater than the lifetime of self, besides the fact that is
            // was properly allocated and initialized.
            unsafe { e.as_mut() }
        })
    }
}

/// A view over a single Environment Tile.
#[derive(Debug)]
pub struct TileView<'a, 'e, I, K, C, T, E> {
    // the ID of the Entity that *owns* this tile
    id: Option<I>,
    // the reference to the Tile in the Environment, where the *weak* references
    // to the entities are stored
    tile: &'a mut Tile<'e, I, K, C, T, E>,
}

impl<'a, 'e: 'a, I: Eq + Clone, K, C, T, E> TileView<'a, 'e, I, K, C, T, E> {
    /// Gets an iterator over the entities located in this Tile that does not
    /// include the entity that *owns* this tile view. The entities are returned
    /// in arbitrary order.
    pub fn entities(
        &self,
    ) -> impl Iterator<Item = &entity::Trait<'e, I, K, C, T, E>> {
        self.tile.entities().filter(move |e| {
            !matches!(&self.id, Some(entity_id) if entity_id == e.id())
        })
    }

    /// Gets an iterator over mutable entities located in this Tile that does not
    /// include the entity that *owns* this tile view. The entities are returned
    /// in arbitrary order.
    pub fn entities_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut entity::Trait<'e, I, K, C, T, E>> {
        let entity_id = self.id.clone();
        self.tile.entities_mut().filter(move |e| {
            !matches!(&entity_id, Some(entity_id) if entity_id == e.id())
        })
    }

    /// Gets the total number of entities located in this Tile, including the
    /// entity that *owns* the tile.
    pub fn len(&self) -> usize {
        self.tile.entities.len()
    }

    /// Returns true only if there are no entities located in this tile.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, 'e, I, K, C, T, E> TileView<'a, 'e, I, K, C, T, E> {
    /// Constructs a new TileView with a specific Entity as owner.
    pub(crate) fn with_owner(
        id: I,
        tile: &'a mut Tile<'e, I, K, C, T, E>,
    ) -> Self {
        Self { id: Some(id), tile }
    }

    /// Gets a reference to the inner Tile.
    pub(crate) fn inner(&self) -> &Tile<'e, I, K, C, T, E> {
        self.tile
    }
}
