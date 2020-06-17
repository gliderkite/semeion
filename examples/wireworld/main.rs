//! https://en.wikipedia.org/wiki/Wireworld

#![allow(clippy::type_complexity)]

use ggez::graphics;
use ggez::*;
use semeion::*;
use std::collections::HashMap;

use entity::cell::State;
use entity::*;
use pattern::*;

mod entity;
mod env;
mod pattern;

struct GameState<'a> {
    // the environment where the simulation takes place
    env: Environment<'a, Id, Kind, Context, graphics::DrawParam, GameError>,
}

/// Cache of mashes per Cell state.
#[derive(Debug)]
pub struct Meshes(HashMap<cell::State, graphics::Mesh>);

impl Meshes {
    /// Constructs a new map of meshes for each Cell state.
    fn new(ctx: &mut Context) -> Result<Self, GameError> {
        let states =
            [State::Conductor, State::ElectronHead, State::ElectronTail];
        let mut meshes = HashMap::with_capacity(states.len());
        for &state in &states {
            meshes.insert(state, state.mesh(ctx)?);
        }
        Ok(Self(meshes))
    }

    /// Gets a Mesh according to the given State.
    pub fn get(&self, state: State) -> Option<&graphics::Mesh> {
        self.0.get(&state)
    }
}

impl<'a> GameState<'a> {
    /// Constructs the game state by populating the environment with the initial
    /// entities.
    fn new() -> Result<Self, GameError> {
        Ok(Self {
            env: Environment::new(env::bounds()),
        })
    }

    /// Draw stats in the bottom-right corner of the screen.
    fn display_stats(&self, ctx: &mut Context) -> GameResult {
        let text = format!("Generation: {:?}", self.env.generation());
        let foreground = graphics::WHITE;
        let fragment = graphics::TextFragment::new(text).color(foreground);
        let text = graphics::Text::new(fragment);
        use ggez::nalgebra::*;
        let dest = Point2::new(env::WIDTH - 150.0, env::HEIGHT - 22.0);
        graphics::draw(ctx, &text, graphics::DrawParam::default().dest(dest))?;
        Ok(())
    }
}

impl<'a> event::EventHandler for GameState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 7) {
            self.env.nextgen()?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.env.draw(ctx, &graphics::DrawParam::default())?;
        self.display_stats(ctx)?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    use ggez::conf::{WindowMode, WindowSetup};

    let (ctx, events_loop) =
        &mut ContextBuilder::new("wireworld", "Marco Conte")
            .window_setup(WindowSetup::default().title("Wireworld!"))
            .window_mode(
                WindowMode::default().dimensions(env::WIDTH, env::HEIGHT),
            )
            .build()?;

    // the cached Cell meshes, shared between all cells as immutable reference
    let meshes = Meshes::new(ctx)?;
    let mut game = GameState::new()?;

    for (location, state) in Pattern::clock() {
        game.env.insert(Cell::new(location, state, &meshes));
    }

    event::run(ctx, events_loop, &mut game)?;
    Ok(())
}
