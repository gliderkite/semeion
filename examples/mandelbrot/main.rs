//! https://en.wikipedia.org/wiki/Mandelbrot_set

use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::*;
use num_complex::Complex;

use semeion::*;

mod entity;
mod env;

struct GameState<'a> {
    // The Environment represent the whole space of the Mandelbrot set, or to be
    // precise the region of the Mandelbrot set that will be displayed, where
    // each Entity occupies always the same tile, and represents a single pixel
    // of the image (and its status will determine its color).
    env: Environment<'a, (), Context>,
    // The current visible complex plane bounds.
    plane: env::Plane,
    // The area of the complex region we want to zoom into.
    zoom_area: Option<graphics::Rect>,
    // The fractal image made up of RGBA individual values.
    image: Vec<u8>,
    // True only if an update of the entities state is required.
    update: bool,
}

impl<'a> GameState<'a> {
    /// Constructs the game state by populating the environment with the initial
    /// entities.
    fn new() -> Self {
        let dimension = env::dimension();
        let mut env = Environment::new(dimension);
        debug_assert!(env.is_empty());

        // populate the whole environment, where each pixel is represented by
        // its own entity
        for y in 0..dimension.y {
            for x in 0..dimension.x {
                let location = Location { x, y };
                let id = location.one_dimensional(dimension);
                env.insert(entity::Pixel::new(id, location));
            }
        }

        Self {
            env,
            plane: env::Plane::default(),
            zoom_area: None,
            image: Vec::with_capacity(4 * dimension.len()),
            update: true,
        }
    }
}

impl<'a> event::EventHandler<ggez::GameError> for GameState<'a> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.update {
            return Ok(());
        }

        self.update = false;
        self.image.clear();

        // compute the state of each pixel after setting the coordinate it
        // represents in the complex plane, according to the current visible
        // plane bounds
        for e in self.env.entities_mut() {
            if let (Some(loc), Some(state)) = (e.location(), e.state_mut()) {
                let state = state
                    .as_any_mut()
                    .downcast_mut::<entity::State>()
                    .expect("Invalid state");
                let point = env::location_to_point(loc, self.plane);
                state.set_point(point);
            }
        }

        self.env
            .nextgen()
            .expect("Cannot move to the next generation");

        // iterate over each pixel to get its current state and its RGBA value
        // that will be pushed into the new image data
        let dimension = env::dimension();
        for y in 0..dimension.y {
            for x in 0..dimension.x {
                let pixel = self
                    .env
                    .entities_at(Location { x, y })
                    .next()
                    .expect("Entity not found");
                let state = pixel
                    .state()
                    .and_then(|s| s.as_any().downcast_ref::<entity::State>())
                    .expect("Invalid state");

                self.image.extend(&state.rgba());
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // draw the fractal image
        if !self.image.is_empty() {
            let dimension = env::dimension();
            let image = graphics::Image::from_rgba8(
                ctx,
                dimension.x as u16,
                dimension.y as u16,
                &self.image,
            )?;
            graphics::draw(ctx, &image, graphics::DrawParam::default())?;
        }

        // draw the zoom area
        if let Some(area) = self.zoom_area {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(2.0),
                area,
                [1.0, 0.0, 0.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if self.zoom_area.is_none() {
            // initialize the zoom area with minimum size
            self.zoom_area = Some(graphics::Rect::new(x, y, 1.0, 1.0));
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let Some(area) = &mut self.zoom_area {
            // scale the zoom area to keep current window aspect ratio, to achieve
            // that keeps the current with and the zoom area center point while
            // changing the area height
            let center = area.y + area.h / 2.0;
            area.h = area.w / env::aspect_ratio();
            let new_center = area.y + area.h / 2.0;
            area.y += (center - new_center).abs() / 2.0;
            debug_assert!((env::aspect_ratio() - area.w / area.h).abs() < 0.01);

            let ratio = (
                self.plane.with() / env::WIDTH as f64,
                self.plane.height() / env::HEIGHT as f64,
            );

            // coordinates of the new plane where to zoom in
            let top_left = self.plane.top_left;
            let x = ratio.0 * area.x as f64 + top_left.re;
            let y = ratio.1 * area.y as f64 + top_left.im;
            let x2 = ratio.0 * (area.x + area.w) as f64 + top_left.re;
            let y2 = ratio.1 * (area.y + area.h) as f64 + top_left.im;

            // update the current plane with the new zoom coordinates
            self.plane = env::Plane {
                top_left: Complex { re: x, im: y },
                bottom_right: Complex { re: x2, im: y2 },
            };

            self.update = true;
            self.zoom_area = None;
        }
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) {
        if let Some(area) = &mut self.zoom_area {
            // update the area dimension according to the mouse position
            area.w = (x - area.x).max(1.0);
            area.h = (y - area.y).max(1.0);
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            self.zoom_area = None;
        }
    }
}

fn main() -> GameResult {
    use ggez::conf::{WindowMode, WindowSetup};

    #[cfg(not(feature = "parallel"))]
    let title = "Mandelbrot!";
    #[cfg(feature = "parallel")]
    let title = "Mandelbrot Parallel!";

    let (ctx, events_loop) = ContextBuilder::new("mandelbrot", "Marco Conte")
        .window_setup(WindowSetup::default().title(title))
        .window_mode(WindowMode::default().dimensions(env::WIDTH, env::HEIGHT))
        .build()?;

    let game = GameState::new();
    event::run(ctx, events_loop, game)
}
