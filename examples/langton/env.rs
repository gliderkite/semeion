use semeion::{Dimension, Size};

/// The width of the window and size of the environment.
pub const WIDTH: f32 = 1000.0;

/// The height of the window and size of the environment.
pub const HEIGHT: f32 = 800.0;

/// The length of each environment grid tile.
pub const SIDE: f32 = 10.0;

/// Gets the size of the environment.
pub fn size() -> Size {
    Size {
        width: WIDTH,
        height: HEIGHT,
    }
}

/// Gets the dimension of the environment.
pub fn dimension() -> Dimension {
    size().to_dimension(SIDE)
}
