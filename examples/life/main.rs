//! https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
//!
//! The Game of Life is based on the following rules:
//! 1. Any live cell with two or three live neighbors survives.
//! 2. Any dead cell with three live neighbors becomes a live cell.
//! 3. All other live cells die in the next generation. Similarly, all other
//!     dead cells stay dead.
//!
//! This implementation does not encode a dead cell as a new entity of a specific
//! kind in the environment, but instead it exploit the scope of the living cells
//! that are allowed to see a portion of the environment 2 tiles beyond their
//! current location, so that for each dead cell in their immediate surroundings
//! (border), it is checked whether there are enough surroundings alive cells,
//! that would allow the dead cell to become alive (as part of the offspring of
//! the current alive cell in question). A cache of locations is shared with all
//! the living cells to check if any of the cells in their border has been already
//! visited during the current generation.

#![allow(clippy::type_complexity)]

use ggez::graphics;
use ggez::*;
use semeion::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use entity::*;
use pattern::*;

mod entity;
mod env;
mod pattern;

struct GameState {
    // the environment where the simulation takes place
    env: Environment<Id, Kind, Context, graphics::DrawParam, GameError>,
    // shared cache for already visited dead cells locations per generation
    visited: Rc<RefCell<HashSet<Location>>>,
}

impl GameState {
    /// Constructs the game state by populating the environment with the initial
    /// entities.
    fn new(ctx: &mut Context) -> Result<Self, GameError> {
        let mut game = Self {
            env: Environment::new(env::bounds()),
            visited: Rc::new(RefCell::new(HashSet::new())),
        };

        // a grid as a static entity used only for drawing purposes in order to
        // show the white grid cells borders
        game.env.insert(Grid::new(grid::mesh(ctx)?));

        // add a list of alive Cells from a predefined pattern
        for location in Pattern::diehard() {
            game.env.insert(Cell::new(
                location,
                cell::mesh(ctx)?,
                Rc::downgrade(&game.visited),
            ));
        }

        Ok(game)
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
        while timer::check_update_time(ctx, 10) {
            self.visited.borrow_mut().clear();
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

    let (ctx, events_loop) = &mut ContextBuilder::new("life", "Marco Conte")
        .window_setup(WindowSetup::default().title("Game of Life!"))
        .window_mode(WindowMode::default().dimensions(env::WIDTH, env::HEIGHT))
        .build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, events_loop, state)?;
    Ok(())
}
