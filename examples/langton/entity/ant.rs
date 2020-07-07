use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameError};

use super::{Cell, Kind};
use crate::env;
use semeion::*;

/// Constructs a new mesh for the Ant.
pub fn mesh(ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
    let mut mesh = graphics::MeshBuilder::new();
    let center = Point2::origin();
    let tolerance = 1.0;
    let radius = env::SIDE / 2.0;

    mesh.circle(
        graphics::DrawMode::fill(),
        center,
        radius,
        tolerance,
        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
    );

    mesh.build(ctx)
}

/// The direction towards where the ant is looking.
#[derive(Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
pub struct Ant<'a> {
    id: Id,
    direction: Direction,
    location: Location,
    mesh: graphics::Mesh,
    offspring_mesh: graphics::Mesh,
    offspring: Offspring<'a, Kind, Context>,
}

impl<'a> Ant<'a> {
    /// Constructs a new ant, with an initial location within the environment.
    pub fn new(
        location: Location,
        mesh: graphics::Mesh,
        offspring_mesh: graphics::Mesh,
    ) -> Self {
        Self {
            // IDs are simply randomly generated as the possibility of collisions
            // are very very low
            id: rand::random(),
            direction: Direction::Left,
            mesh,
            location,
            offspring_mesh,
            offspring: Offspring::default(),
        }
    }

    /// Turn the Ant 90° clockwise and move forwards of one tile.
    fn turn_right_and_move_forward(&mut self) {
        let (offset, direction) = match self.direction {
            Direction::Left => ((0, -1), Direction::Up),
            Direction::Up => ((1, 0), Direction::Right),
            Direction::Right => ((0, 1), Direction::Down),
            Direction::Down => ((-1, 0), Direction::Left),
        };
        self.direction = direction;
        self.location.translate(offset.into(), env::dimension());
    }

    /// Turn the Ant 90° counter-clockwise and move forwards of one tile.
    fn turn_left_and_move_forward(&mut self) {
        let (offset, direction) = match self.direction {
            Direction::Up => ((-1, 0), Direction::Left),
            Direction::Right => ((0, -1), Direction::Up),
            Direction::Down => ((1, 0), Direction::Right),
            Direction::Left => ((0, 1), Direction::Down),
        };
        self.direction = direction;
        self.location.translate(offset.into(), env::dimension());
    }
}

impl<'a> Entity<'a> for Ant<'a> {
    type Kind = Kind;
    type Context = Context;

    fn id(&self) -> Id {
        self.id
    }

    fn kind(&self) -> Self::Kind {
        Kind::Ant
    }

    fn location(&self) -> Option<Location> {
        Some(self.location)
    }

    fn scope(&self) -> Option<Scope> {
        // The Ant can only see the tile it's currently in, it has no scope
        // beyond it.
        Some(Scope::empty())
    }

    fn lifespan(&self) -> Option<Lifespan> {
        // The lifespan of the Ant is infinite.
        Some(Lifespan::Immortal)
    }

    /// The Ant behaves according to the rules below:
    /// - At a white square, turn 90° clockwise, flip the color of the square,
    ///     move forward one unit.
    /// - At a black square, turn 90° counter-clockwise, flip the color of the
    ///     square, move forward one unit.
    fn react(
        &mut self,
        neighborhood: Option<Neighborhood<Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        // given the scope of the Ant, we expect the seeable portion of the
        // environment to be just the tile where the Ant is currently located
        let mut neighborhood = neighborhood.expect("Unexpected neighborhood");
        let tile = neighborhood.center_mut();
        // the tile in question can either be BLACK or WHITE, we encode this
        // information with a Cell entity or no entity respectively
        let mut entities = tile.entities_mut();
        let black_cell = entities.find(|e| e.kind() == Kind::Cell);

        if let Some(cell) = black_cell {
            // if the cell is BLACK, we flip its color by "killing" the entity
            // reducing its lifespan to 0 and move left
            debug_assert_eq!(cell.location(), self.location());
            let lifespan = cell.lifespan_mut().expect("Invalid Cell lifespan");
            lifespan.clear();

            self.turn_left_and_move_forward();
        } else {
            // if the cell is WHITE, we flip its color by creating a new entity
            // as offspring for the next generation, and move right
            let black_cell =
                Cell::new(self.location, self.offspring_mesh.clone());
            self.offspring.insert(black_cell);

            self.turn_right_and_move_forward();
        }

        Ok(())
    }

    fn offspring(
        &mut self,
    ) -> Option<Offspring<'a, Self::Kind, Self::Context>> {
        // release the offspring (if any) to the environment
        Some(self.offspring.drain())
    }

    fn draw(
        &self,
        ctx: &mut Self::Context,
        transform: Transform,
    ) -> Result<(), Error> {
        // Draw the shape of the Ant without taking into consideration the
        // given transformation (that is always going to be equal to the Identity
        // matrix) since for the purposes of this simulation neither zoom or
        // panning are supported.
        debug_assert_eq!(transform, Transform::identity());

        // the radius is equal to half the grid tiles side
        let radius = env::SIDE / 2.0;
        // coordinate in pixels of the top-left corner of the mesh
        let location = self.location.to_pixel_coords(env::SIDE);
        let location = Point2::new(location.x, location.y);
        // shift the center of the shape to the center of the tile
        let offset = location + Vector2::new(radius, radius);

        let param = graphics::DrawParam::default();
        graphics::draw(ctx, &self.mesh, param.dest(offset))
            .map_err(Error::with_message)
    }
}
