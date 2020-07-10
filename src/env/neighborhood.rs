use std::collections::HashSet;

use super::*;

/// The neighbor tiles of a specific Entity.
#[derive(Debug)]
pub struct Neighborhood<'a, 'e, K, C> {
    dimension: Dimension,
    tiles: Vec<TileView<'a, 'e, K, C>>,
}

impl<'a, 'e, K, C> Neighborhood<'a, 'e, K, C> {
    /// Gets the dimension of this neighborhood.
    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    /// Gets an iterator over all the Tiles that belong to this Neighborhood.
    pub fn tiles(&self) -> impl Iterator<Item = &TileView<'a, 'e, K, C>> {
        self.tiles.iter()
    }

    /// Gets an iterator over all the mutable Tiles that belong to this
    /// Neighborhood.
    pub fn tiles_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut TileView<'a, 'e, K, C>> {
        self.tiles.iter_mut()
    }

    /// Gets a reference to the Tile located at the given offset from the center
    /// of this Neighborhood.
    ///
    /// The Neighborhood is seen as a Torus from this method, therefore, out of
    /// bounds offsets will be translated considering that the Neighborhood edges
    /// are joined.
    pub fn tile(&self, offset: Offset) -> &TileView<'a, 'e, K, C> {
        &self.tiles[self.index(offset)]
    }

    /// Gets a mutable reference to the Tile located at the given offset from
    /// the center of this Neighborhood.
    ///
    /// The Neighborhood is seen as a Torus from this method, therefore, out of
    /// bounds offsets will be translated considering that the Neighborhood edges
    /// are joined.
    pub fn tile_mut(&mut self, offset: Offset) -> &mut TileView<'a, 'e, K, C> {
        debug_assert!(!self.tiles.is_empty());
        let index = self.index(offset);
        &mut self.tiles[index]
    }

    /// Gets a reference to the Tile located in the center of this Neighborhood.
    pub fn center(&self) -> &TileView<'a, 'e, K, C> {
        self.tile(Offset::origin())
    }

    /// Gets a mutable reference to the Tile located in the center of this
    /// Neighborhood.
    pub fn center_mut(&mut self) -> &mut TileView<'a, 'e, K, C> {
        self.tile_mut(Offset::origin())
    }

    /// Gets a list of tiles that surround the Tile T of this Neighborhood,
    /// located at a given Offset from the center Tile, and according to the
    /// given Scope, that represents the distance from the Tile T.
    ///
    /// The tiles are returned in arbitrary order. Returns None if any of the
    /// border tiles is out of the Neighborhood dimension for the given Scope.
    pub fn border(
        &self,
        offset: Offset,
        scope: Scope,
    ) -> Option<Vec<&TileView<'a, 'e, K, C>>> {
        // the location of the tile T relative to the center of the Neighborhood
        let loc = self.dimension.center() + offset;

        // iterate over the 4 corners surrounding the tile T to check if
        // the whole border of the tile T is contained within this Neighborhood
        // according to the given scope
        for &delta in &Offset::corners(scope) {
            if !self.dimension.contains(loc + delta) {
                return None;
            }
        }

        let mut tiles =
            Vec::with_capacity(Dimension::perimeter_with_scope(scope));
        for mut delta in Offset::border(scope) {
            let loc = *delta.translate(offset, self.dimension);
            tiles.push(self.tile(loc))
        }

        debug_assert_eq!(tiles.capacity(), tiles.len());
        Some(tiles)
    }

    /// Gets the index of the Tile located at the given offset from the center
    /// of this Neighborhood. The Neighborhood is seen as a Torus from this
    /// method, therefore, out of bounds offsets will be translated
    /// considering that the Neighborhood edges are joined.
    fn index(&self, offset: Offset) -> usize {
        debug_assert!(!self.tiles.is_empty());
        let mut center = self.dimension.center();
        let index = center
            .translate(offset, self.dimension)
            .one_dimensional(self.dimension);
        debug_assert!(index < self.tiles.len());
        index
    }

    /// Returns true only if this Neighborhood contains unique Tiles.
    fn is_unique(&self) -> bool {
        let mut uniq = HashSet::with_capacity(self.tiles.len());
        self.tiles
            .iter()
            .all(move |tile| uniq.insert(tile.inner() as *const Tile<'e, K, C>))
    }
}

impl<'a, 'e, K, C> From<Vec<TileView<'a, 'e, K, C>>>
    for Neighborhood<'a, 'e, K, C>
{
    /// Constructs a new Neighborhood from a list of tiles that can encode a
    /// squared grid.
    fn from(tiles: Vec<TileView<'a, 'e, K, C>>) -> Self {
        debug_assert!(!tiles.is_empty());
        let length = tiles.len() as f64;
        // NeighborHoods can only be constructed if they represent squares
        debug_assert!(math::is_perfect_square(length));

        let side = length.sqrt() as i32;
        let neighborhood = Self {
            tiles,
            dimension: Dimension { x: side, y: side },
        };

        // NeighborHoods can only contain unique Tiles
        debug_assert!(neighborhood.is_unique());
        neighborhood
    }
}
