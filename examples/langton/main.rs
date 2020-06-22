//! https://en.wikipedia.org/wiki/Langton%27s_ant
#![allow(clippy::type_complexity)]

use ggez::graphics;
use ggez::*;
use semeion::*;

use entity::*;

mod entity;
mod env;

struct GameState {
    // the environment where the simulation takes place
    env:
        Environment<'static, Id, Kind, Context, graphics::DrawParam, GameError>,
}

impl GameState {
    /// Constructs the game state by populating the environment with the initial
    /// entities.
    fn new(ctx: &mut Context) -> Result<Self, GameError> {
        let mut env = Environment::new(env::dimension());
        // a grid as a static entity used only for drawing purposes in order to
        // show the white grid cells borders
        env.insert(Grid::new(grid::mesh(ctx)?));

        // the ant, placed in the center of the environment
        let location = env::dimension().center();
        env.insert(Ant::new(location, ant::mesh(ctx)?, cell::mesh(ctx)?));

        Ok(Self { env })
    }

    /// Draw stats in the bottom-right corner of the screen.
    fn display_stats(&self, ctx: &mut Context) -> GameResult {
        let text = format!("Generation: {:?}", self.env.generation());
        let foreground = graphics::Color::new(0.1, 0.2, 0.3, 3.0);
        let fragment = graphics::TextFragment::new(text).color(foreground);
        let text = graphics::Text::new(fragment);
        use ggez::nalgebra::*;
        let dest = Point2::new(env::WIDTH - 150.0, env::HEIGHT - 22.0);
        graphics::draw(ctx, &text, graphics::DrawParam::default().dest(dest))?;
        Ok(())
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            self.env.nextgen()?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.9, 0.9, 0.9, 1.0].into());
        self.env.draw(ctx, &graphics::DrawParam::default())?;
        self.display_stats(ctx)?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    use ggez::conf::{WindowMode, WindowSetup};

    let (ctx, events_loop) = &mut ContextBuilder::new("langton", "Marco Conte")
        .window_setup(WindowSetup::default().title("Langton Ant!"))
        .window_mode(WindowMode::default().dimensions(env::WIDTH, env::HEIGHT))
        .build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, events_loop, state)?;
    Ok(())
}
