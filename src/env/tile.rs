use super::*;

/// A 1-dimensional list of tiles that represents a grid of given dimension with
/// squared tiles of the same side length.
/// Only entities that have a defined location will be stored in this data
/// structure.
#[derive(Debug)]
pub struct Tiles<'e, K, C> {
    dimension: Dimension,
    tiles: Vec<Tile<'e, K, C>>,
}

impl<'e, K, C> Tiles<'e, K, C> {
    /// Constructs a new list of tiles of the given dimension with no entities
    /// assigned to it.
    pub fn new(dimension: impl Into<Dimension>) -> Self {
        let dimension = dimension.into();
        let mut tiles = Vec::with_capacity(dimension.len());
        for i in 0..dimension.len() {
            tiles.push(Tile::new(Location::from_one_dimensional(i, dimension)));
        }

        Self { dimension, tiles }
    }

    /// Gets the Dimension of the Environment.
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// Inserts the given Entity in the grid according to its location. If the
    /// Entity has not location it will not be inserted.
    /// Returns whether the Entity was inserted or not.
    pub fn insert(&mut self, entity: &mut EntityTrait<'e, K, C>) -> bool {
        if let Some(location) = entity.location() {
            let index = location.one_dimensional(self.dimension);
            debug_assert!(index < self.tiles.len());
            let tile = &mut self.tiles[index];
            tile.entities
                .insert(entity.id(), entity as *mut EntityTrait<'e, K, C>);
            true
        } else {
            false
        }
    }

    /// Remove the Entity with the given ID from the given location.
    /// Returns whether the Entity was removed or not.
    pub fn remove(&mut self, id: Id, location: impl Into<Location>) -> bool {
        let location = location.into();
        let index = location.one_dimensional(self.dimension);
        debug_assert!(index < self.tiles.len());
        let tile = &mut self.tiles[index];
        tile.entities.remove(&id).is_some()
    }

    /// Move the Entity with the given ID between a previous and a new location.
    pub fn relocate(
        &mut self,
        id: Id,
        from: impl Into<Location>,
        to: impl Into<Location>,
    ) {
        let from = from.into();
        let index = from.one_dimensional(self.dimension);
        debug_assert!(index < self.tiles.len());
        let tile = &mut self.tiles[index];

        if let Some(e) = tile.entities.remove(&id) {
            let to = to.into();
            let index = to.one_dimensional(self.dimension);
            let tile = &mut self.tiles[index];
            tile.entities.insert(id, e);
        }
    }

    /// Gets an iterator over all the entities located at the given location.
    ///
    /// The Environment is seen as a Torus from this method, therefore, out of
    /// bounds offsets will be translated considering that the Environment
    /// edges are joined.
    pub fn entities_at(
        &self,
        location: impl Into<Location>,
    ) -> impl Iterator<Item = &EntityTrait<'e, K, C>> {
        self.tile_at(location.into()).entities()
    }

    /// Gets an iterator over all the (mutable) entities located at the given
    /// location.
    ///
    /// The Environment is seen as a Torus from this method, therefore, out of
    /// bounds offsets will be translated considering that the Environment
    /// edges are joined.
    pub fn entities_at_mut(
        &mut self,
        location: impl Into<Location>,
    ) -> impl Iterator<Item = &mut EntityTrait<'e, K, C>> {
        self.tile_at_mut(location.into()).entities_mut()
    }

    /// Gets the tile at the given location.
    fn tile_at(&self, location: Location) -> &Tile<'e, K, C> {
        let index = self.tile_index_at(location);
        let tile = &self.tiles[index];
        debug_assert_eq!(tile.location, location);
        tile
    }

    /// Gets the (mutable) tile at the given location.
    fn tile_at_mut(&mut self, location: Location) -> &mut Tile<'e, K, C> {
        let index = self.tile_index_at(location);
        let tile = &mut self.tiles[index];
        debug_assert_eq!(tile.location, location);
        tile
    }

    /// Gets the tile index at the given location.
    fn tile_index_at(&self, location: Location) -> usize {
        let index = location.one_dimensional(self.dimension);
        debug_assert!(index < self.tiles.len());
        index
    }

    /// Gets the area of the environment surrounding the given Entity.
    /// Returns None if the Entity has no location or scope, or if the scope of
    /// the Entity forces its neighborhood to wrap onto itself due to the
    /// dimensions of the Environment being not big enough to contain it.
    pub fn neighborhood(
        &self,
        entity: &EntityTrait<'e, K, C>,
    ) -> Option<Neighborhood<'_, 'e, K, C>> {
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

                // build the portion of the environment seen by the entity tile
                // by tile from the top-left corner to the bottom-down corner
                for y in -scope..=scope {
                    for x in -scope..=scope {
                        let mut location = center;
                        location.translate(Offset { x, y }, self.dimension);
                        let index = location.one_dimensional(self.dimension);
                        debug_assert!(index < self.tiles.len());

                        let tile = &self.tiles[index];
                        neighborhood
                            .push(TileView::with_owner(entity.id(), tile));
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
pub struct Tile<'e, K, C> {
    // the location of the Tile in the Environment
    location: Location,
    // the entities that currently occupy this Tile
    entities: HashMap<Id, *mut EntityTrait<'e, K, C>>,
}

impl<'e, K, C> Tile<'e, K, C> {
    /// Constructs a new Tile with the given Location and no entities.
    fn new(location: impl Into<Location>) -> Self {
        Self {
            location: location.into(),
            entities: HashMap::default(),
        }
    }

    /// Gets an iterator over all the entities located in this Tile.
    /// The entities are returned in arbitrary order.
    pub fn entities(&self) -> impl Iterator<Item = &EntityTrait<'e, K, C>> {
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
        &self,
    ) -> impl Iterator<Item = &mut EntityTrait<'e, K, C>> {
        self.entities.iter().filter_map(move |(_id, e)| {
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

/// A single Environment tile as seen by a single Entity.
#[derive(Debug)]
pub struct TileView<'a, 'e, K, C> {
    // the ID of the Entity that is seeing this tile
    id: Option<Id>,
    // the reference to the Tile in the Environment, where the *weak* references
    // to the entities are stored
    tile: &'a Tile<'e, K, C>,
}

impl<'a, 'e, K, C> TileView<'a, 'e, K, C> {
    /// Gets the Location of this Tile within the Environment.
    pub fn location(&self) -> Location {
        self.tile.location
    }

    /// Gets an iterator over all the entities located in this Tile that does not
    /// include the Entity that is seeing the tile.
    ///
    /// The entities are returned in arbitrary order.
    pub fn entities(&self) -> impl Iterator<Item = &EntityTrait<'e, K, C>> {
        self.tile.entities().filter(move |e| {
            !matches!(&self.id, Some(entity_id) if entity_id == &e.id())
        })
    }

    /// Gets an iterator over all the mutable entities located in this Tile that
    /// does not include the Entity that is seeing the tile.
    ///
    /// The entities are returned in arbitrary order.
    pub fn entities_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut EntityTrait<'e, K, C>> {
        let entity_id = self.id;
        self.tile.entities_mut().filter(move |e| {
            !matches!(&entity_id, Some(entity_id) if entity_id == &e.id())
        })
    }

    /// Gets the total number of entities located in this Tile, including the
    /// Entity that is seeing the tile.
    pub fn count(&self) -> usize {
        self.tile.entities.len()
    }

    /// Returns true only if there are no entities located in this tile.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

impl<'a, 'e, K: PartialEq, C> TileView<'a, 'e, K, C> {
    /// Returns true only if this Tile contains an Entity of the given Kind,
    /// without considering the Entity that is seeing the tile.
    pub fn contains_kind(&self, kind: K) -> bool {
        self.entities().any(|e| e.kind() == kind)
    }

    /// Gets the total number of entities in this Tile of the given Kind,
    /// without considering the Entity that is seeing the tile.
    pub fn count_kind(&self, kind: K) -> usize {
        self.entities().filter(|e| e.kind() == kind).count()
    }
}

impl<'a, 'e, K, C> TileView<'a, 'e, K, C> {
    /// Constructs a new TileView with a specific Entity as owner.
    pub(crate) fn with_owner(id: Id, tile: &'a Tile<'e, K, C>) -> Self {
        Self { id: Some(id), tile }
    }

    /// Gets a reference to the inner Tile.
    pub(crate) fn inner(&self) -> &Tile<'e, K, C> {
        self.tile
    }
}
