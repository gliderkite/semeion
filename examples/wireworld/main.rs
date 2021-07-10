//! https://en.wikipedia.org/wiki/Wireworld

use ggez::*;
use semeion::*;
use std::{collections::HashMap, rc::Rc};

use entity::cell::State;
use entity::*;
use pattern::*;

mod entity;
mod env;
mod pattern;

struct GameState<'a> {
    // the environment where the simulation takes place
    env: Environment<'a, Kind, Context>,
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
    fn new() -> Self {
        let env = Environment::new(env::dimension());
        debug_assert!(env.is_empty());

        Self { env }
    }

    /// Draw stats in the bottom-right corner of the screen.
    fn display_stats(&self, ctx: &mut Context) -> GameResult {
        use mint::Point2;

        let text = format!("Generation: {:?}", self.env.generation());
        let foreground = graphics::Color::WHITE;
        let fragment = graphics::TextFragment::new(text).color(foreground);
        let text = graphics::Text::new(fragment);

        let dest = Point2 {
            x: env::WIDTH - 150.0,
            y: env::HEIGHT - 22.0,
        };
        graphics::draw(ctx, &text, graphics::DrawParam::default().dest(dest))
    }
}

impl<'a> event::EventHandler<GameError> for GameState<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 7) {
            self.env
                .nextgen()
                .expect("Cannot move to the next generation");
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
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

    let (mut ctx, events_loop) =
        ContextBuilder::new("wireworld", "Marco Conte")
            .window_setup(WindowSetup::default().title("Wireworld!"))
            .window_mode(
                WindowMode::default().dimensions(env::WIDTH, env::HEIGHT),
            )
            .build()?;

    // the cached Cell meshes, shared between all cells as immutable reference
    let meshes = Rc::new(Meshes::new(&mut ctx)?);
    let mut game = GameState::new();

    for (location, state) in Pattern::clock() {
        game.env
            .insert(Cell::new(location, state, Rc::clone(&meshes)));
    }

    event::run(ctx, events_loop, game)
}
