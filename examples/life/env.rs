use semeion::{Dimension, Size};

/// The width of the window and size of the environment.
pub const WIDTH: f32 = 1400.0;

/// The height of the window and size of the environment.
pub const HEIGHT: f32 = 900.0;

/// The length of each environment grid tile.
pub const SIDE: f32 = 5.0;

/// Gets the size of the environment.
pub const fn size() -> Size {
    Size {
        width: WIDTH,
        height: HEIGHT,
    }
}

/// Gets the dimension of the environment.
pub fn dimension() -> Dimension {
    size().to_dimension(SIDE)
}
