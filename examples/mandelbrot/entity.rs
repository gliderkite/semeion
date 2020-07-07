use ggez::Context;
use num_complex::Complex;
use std::any::Any;

use crate::env;
use semeion::*;

/// The State of each Pixel Entity defines its color from an arbitrary palette
/// of up to 256 colors.
#[derive(Debug, Default, Clone, Copy)]
pub struct State {
    // the current value of this State
    value: u8,
    // the point of the fractal set the Pixel represent
    point: Complex<f64>,
}

/// Implement the entity::State trait to allow downcasting when querying the
/// Cell state via the Entity::state() method.
impl entity::State for State {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl State {
    /// Gets the RGBA value that represents this State.
    pub fn rgba(&self) -> [u8; 4] {
        let r = (self.value as u32 * 15) as u8;
        let g = (self.value as u32 * 10) as u8;
        let b = (self.value as u32 * 5) as u8;
        let a = 255;
        [r, g, b, a]
    }

    /// Sets the coordinates of the Pixel point in the complex plane.
    pub fn set_point(&mut self, point: Complex<f64>) {
        self.point = point
    }

    /// Tries to determine if the point is in the Mandelbrot set, using at most
    /// limit iterations to decide.
    /// If the point is not a member, return Some(i), where i is the number of
    /// iterations it took for it to leave the circle of radius two centered on
    /// the origin. If the point seems to be a member (more precisely, if we
    /// reached the iteration limit without being able to prove that it is not a
    /// member), return None.
    fn escape_time(&self, limit: u32) -> Option<u32> {
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..limit {
            z = z * z + self.point;
            if z.norm() > 2.0 {
                return Some(i);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Pixel {
    // The ID of the entities can represent the position in the space, since
    // entities will always occupy the whole environment and never move.
    id: Id,
    state: State,
    // the location of the entity relative to the Environment
    location: Location,
}

impl Pixel {
    /// Constructs a new pixel.
    pub fn new(id: Id, location: Location) -> Self {
        Self {
            id,
            state: State::default(),
            location,
        }
    }
}

impl<'a> Entity<'a> for Pixel {
    type Kind = ();
    type Context = Context;

    fn id(&self) -> Id {
        self.id
    }

    fn kind(&self) -> Self::Kind {}

    fn location(&self) -> Option<Location> {
        Some(self.location)
    }

    fn state(&self) -> Option<&dyn entity::State> {
        Some(&self.state)
    }

    fn state_mut(&mut self) -> Option<&mut dyn entity::State> {
        Some(&mut self.state)
    }

    fn react(
        &mut self,
        _: Option<Neighborhood<Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        // compute the next value of the pixel state according to its escape time
        let time = self.state.escape_time(env::ESCAPE_TIME_LIMIT);
        self.state.value = if let Some(time) = time {
            // this pixel belongs to the set, assign an arbitrary but proportional
            // value to the pixel state, according to how long it took to
            // determined it was part of the set
            let step = u8::max_value() as f32 / env::ESCAPE_TIME_LIMIT as f32;
            u8::max_value() - ((time as f32 * step) as u8)
        } else {
            // this pixel doesn't belong to the set
            u8::default()
        };

        Ok(())
    }
}
