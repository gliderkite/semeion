//! https://en.wikipedia.org/wiki/Elementary_cellular_automaton

use ggez::*;
use semeion::*;

use cell::Cell;
use context::{Context, Kind, Rule};

mod cell;
mod context;
mod env;
mod palette;

struct GameState<'a> {
    // the environment where the simulation takes place
    env: Environment<'a, Kind, ggez::Context>,
}

impl<'a> GameState<'a> {
    /// Constructs the game state by populating the environment with the initial
    /// entities.
    fn new(context: &'a Context) -> GameResult<Self> {
        let mut env = Environment::new(env::dimension());
        debug_assert!(env.is_empty());

        // insert the first generation of cells in the top row, with a single alive
        // cell placed in the center
        let dimensions = env::dimension();
        for x in 0..dimensions.x {
            let state = if x == dimensions.center().x {
                cell::State::Alive
            } else {
                cell::State::Dead
            };
            env.insert(Cell::new((x, 0), state, context));
        }

        Ok(Self { env })
    }
}

impl<'a> event::EventHandler for GameState<'a> {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        self.env
            .nextgen()
            .expect("Cannot move to the next generation");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, [0.9, 0.9, 0.9, 1.0].into());
        self.env
            .draw(ctx, Transform::identity())
            .expect("Cannot draw the environment");
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    use ggez::conf::{WindowMode, WindowSetup};

    let mut args: Vec<String> = std::env::args().collect();
    let rule: Rule = args.remove(1).parse().expect("Invalid rule");

    let (ctx, events_loop) = &mut ContextBuilder::new("rule", "Marco Conte")
        .window_setup(WindowSetup::default().title(&format!("Rule {}!", rule)))
        .window_mode(WindowMode::default().dimensions(env::WIDTH, env::HEIGHT))
        .build()?;

    let context = Context::new(rule, ctx)?;
    let state = &mut GameState::new(&context)?;
    event::run(ctx, events_loop, state)?;
    Ok(())
}
