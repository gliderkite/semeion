//! Zoom and pan transformations driven by user inputs.
//!
//! Zoom with keys Up and Down.
//! Pan with A, W, D, and S.

use ggez::input::keyboard::*;
use ggez::*;
use semeion::*;

use entity::*;

mod entity;
mod env;

struct GameState<'a> {
    // the environment where the simulation takes place
    env: Environment<'a, Kind, Context>,
    // the global transformation matrix
    transform: Transform,
}

impl<'a> GameState<'a> {
    /// Constructs the game state by populating the environment with the initial
    /// entities.
    fn new(
        ctx: &mut Context,
        rect_mesh: &'a graphics::Mesh,
    ) -> Result<Self, GameError> {
        let mut env = Environment::new(env::dimension());
        debug_assert!(env.is_empty());
        // a grid as a static entity used only for drawing purposes in order to
        // show the white grid cells borders
        env.insert(Grid::new(grid::mesh(ctx)?));
        env.insert(Rect::new(env::dimension().center(), rect_mesh));

        Ok(Self {
            env,
            transform: Transform::scale_around(
                [0.9, 0.9],
                env::size().center(),
            ),
        })
    }
}

impl<'a> event::EventHandler for GameState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 10) {
            self.env
                .nextgen()
                .expect("Cannot move to the next generation");
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.9, 0.9, 0.9, 1.0].into());
        self.env
            .draw(ctx, self.transform)
            .expect("Cannot draw the environment");
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // compute the translation matrix
        let t = match keycode {
            KeyCode::A => Transform::translate([10.0, 0.0]),
            KeyCode::D => Transform::translate([-10.0, 0.0]),
            KeyCode::W => Transform::translate([0.0, 10.0]),
            KeyCode::S => Transform::translate([0.0, -10.0]),
            _ => Transform::identity(),
        };

        // compute the scaling matrix around the center of the window
        let center = env::size().center();
        let s = match keycode {
            KeyCode::Up => Transform::scale_around([1.1, 1.1], center),
            KeyCode::Down => Transform::scale_around([0.9, 0.9], center),
            _ => Transform::identity(),
        };

        self.transform *= t * s;
    }
}

fn main() -> GameResult {
    use ggez::conf::{WindowMode, WindowSetup};

    let (ctx, events_loop) = &mut ContextBuilder::new("camera", "Marco Conte")
        .window_setup(WindowSetup::default().title("Camera!"))
        .window_mode(WindowMode::default().dimensions(env::WIDTH, env::HEIGHT))
        .build()?;

    let rect_mesh = rect::mesh(ctx)?;

    let state = &mut GameState::new(ctx, &rect_mesh)?;
    event::run(ctx, events_loop, state)?;
    Ok(())
}
