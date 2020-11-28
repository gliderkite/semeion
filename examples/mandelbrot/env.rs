use num_complex::Complex;

use semeion::{Dimension, Location, Size};

/// The width of the window and size of the environment.
pub const WIDTH: f32 = 1600.0;

/// The height of the window and size of the environment.
pub const HEIGHT: f32 = 935.0;

/// The length of each environment grid tile. Each entity represents a single
/// pixel in the screen.
pub const SIDE: f32 = 1.0;

/// The escape time limit used when checking if a point in the complex plane
/// belongs to the Mandelbrot set or not.
pub const ESCAPE_TIME_LIMIT: u32 = 100;

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

/// Gets the window aspect ratio.
pub fn aspect_ratio() -> f32 {
    let ratio = WIDTH / HEIGHT;
    debug_assert!((ratio - dimension().aspect_ratio()).abs() < f32::EPSILON);
    ratio
}

/// The complex plane that defines the bounds of the current Mandelbrot set
/// region.
#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub top_left: Complex<f64>,
    pub bottom_right: Complex<f64>,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            top_left: Complex { re: -2.5, im: -1.5 },
            bottom_right: Complex { re: 1.0, im: 1.5 },
        }
    }
}

impl Plane {
    pub fn with(&self) -> f64 {
        (self.bottom_right.re - self.top_left.re).abs()
    }

    pub fn height(&self) -> f64 {
        (self.top_left.im - self.bottom_right.im).abs()
    }
}

/// Given the row and column of a pixel in the output image, returns the
/// corresponding point on the complex plane.
pub fn location_to_point(location: Location, plane: Plane) -> Complex<f64> {
    let x = location.x as f64 * SIDE as f64;
    let y = location.y as f64 * SIDE as f64;

    Complex {
        re: plane.top_left.re + x * plane.with() / WIDTH as f64,
        im: plane.top_left.im + y * plane.height() / HEIGHT as f64,
    }
}
