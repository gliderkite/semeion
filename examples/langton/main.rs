//! https://en.wikipedia.org/wiki/Langton%27s_ant

use ggez::*;
use semeion::*;

use entity::*;

mod entity;
mod env;

struct GameState<'a> {
    // the environment where the simulation takes place
    env: Environment<'a, Kind, Context>,
}

impl<'a> GameState<'a> {
    /// Constructs the game state by populating the environment with the initial
    /// entities.
    fn new(ctx: &mut Context) -> Result<Self, GameError> {
        let mut env = Environment::new(env::dimension());
        debug_assert!(env.is_empty());
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
        use mint::Point2;

        let text = format!("Generation: {:?}", self.env.generation());
        let foreground = graphics::Color::new(0.1, 0.2, 0.3, 3.0);
        let fragment = graphics::TextFragment::new(text).color(foreground);
        let text = graphics::Text::new(fragment);

        let dest = Point2 {
            x: env::WIDTH - 150.0,
            y: env::HEIGHT - 22.0,
        };
        graphics::draw(ctx, &text, graphics::DrawParam::default().dest(dest))?;
        Ok(())
    }
}

impl<'a> event::EventHandler<GameError> for GameState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            self.env
                .nextgen()
                .expect("Cannot move to the next generation");
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.9, 0.9, 0.9, 1.0].into());
        self.env
            .draw(ctx, Transform::identity())
            .expect("Cannot draw the environment");
        self.display_stats(ctx)?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    use conf::{WindowMode, WindowSetup};

    let (mut ctx, events_loop) = ContextBuilder::new("langton", "Marco Conte")
        .window_setup(WindowSetup::default().title("Langton Ant!"))
        .window_mode(WindowMode::default().dimensions(env::WIDTH, env::HEIGHT))
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}
