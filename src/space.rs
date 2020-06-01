/// A Point in 2D space.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

/// Represents the bounds of a grid as the integer number of columns and rows.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bounds {
    pub x: i32,
    pub y: i32,
}

/// The size of a Shape represented as number of pixels.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// Represents the location of an entity within the environment as pair of
/// coordinate that identify the environment grid tile.
pub type Location = Point<i32>;

/// Represents the location of an entity within the environment expressed in
/// pixel coordinates.
pub type PixelCoordinate = Point<f32>;

impl PixelCoordinate {
    /// Gets the origin coordinates in (0.0, 0.0).
    pub const fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Location {
    /// Gets the origin coordinates in (0, 0).
    pub const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Converts the Point into a point expressed as pixel coordinates, according
    /// to the length of each grid square side.
    pub fn to_pixel_coords(self, side: f32) -> PixelCoordinate {
        PixelCoordinate {
            x: self.x as f32 * side,
            y: self.y as f32 * side,
        }
    }

    /// Maps a 2-dimensional coordinate to a 1-dimensional index.
    pub fn one_dimensional(self, bounds: Bounds) -> usize {
        debug_assert!(!self.x.is_negative());
        debug_assert!(!self.y.is_negative());
        let pos = self.y.saturating_mul(bounds.y).saturating_add(self.x);
        debug_assert!(!pos.is_negative());
        debug_assert!(pos < bounds.x.saturating_mul(bounds.y));
        pos as usize
    }

    /// Translates the current location by the given offset, while keeping the
    /// final location within a Torus with the given bounds.
    pub fn translate(&mut self, offset: Self, bounds: Bounds) -> &mut Self {
        self.x = self.x.saturating_add(offset.x).rem_euclid(bounds.x);
        self.y = self.y.saturating_add(offset.y).rem_euclid(bounds.y);
        self
    }
}

impl From<(i32, i32)> for Location {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Size {
    /// Converts the Size to a Bounds according to the given side length.
    pub fn to_bounds(self, side: f32) -> Bounds {
        Bounds {
            x: (self.width / side) as i32,
            y: (self.height / side) as i32,
        }
    }
}

impl Bounds {
    /// Gets the number of elements in a grid of given Bounds, equal to the
    /// number of row by the number of columns.
    pub fn len(self) -> usize {
        debug_assert!(!self.x.is_negative());
        debug_assert!(!self.y.is_negative());
        self.x.saturating_mul(self.y) as usize
    }

    /// Returns true only if the number of elements in the grid is 0.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
}
