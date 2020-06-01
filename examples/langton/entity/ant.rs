use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameError};

use super::{Cell, Id, Kind};
use crate::env;
use semeion::*;

/// The ID of the Ant.
const ID: Id = -2;

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
pub struct Ant {
    id: Id,
    direction: Direction,
    location: Location,
    mesh: graphics::Mesh,
    offspring_mesh: graphics::Mesh,
    offspring:
        Offspring<'static, Id, Kind, Context, graphics::DrawParam, GameError>,
    offspring_id: Id,
}

impl Ant {
    /// Constructs a new ant, with an initial location within the environment.
    pub fn new(
        location: Location,
        mesh: graphics::Mesh,
        offspring_mesh: graphics::Mesh,
    ) -> Self {
        Self {
            id: ID,
            direction: Direction::Left,
            mesh,
            location,
            offspring_mesh,
            offspring: Offspring::default(),
            offspring_id: ID + 1,
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
        self.location.translate(offset.into(), env::bounds());
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
        self.location.translate(offset.into(), env::bounds());
    }
}

impl Entity<'static> for Ant {
    type Id = Id;
    type Kind = Kind;
    type Context = Context;
    type Transform = graphics::DrawParam;
    type Error = GameError;

    fn id(&self) -> &Self::Id {
        &self.id
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

    fn lifespan_mut(&mut self) -> Option<&mut Lifespan> {
        // No other entity can affect the Ant lifespan.
        None
    }

    /// The Ant behaves according to the rules below:
    /// - At a white square, turn 90° clockwise, flip the color of the square,
    ///     move forward one unit.
    /// - At a black square, turn 90° counter-clockwise, flip the color of the
    ///     square, move forward one unit.
    fn act(
        &mut self,
        neighborhood: Option<
            NeighborHood<
                Self::Id,
                Self::Kind,
                Self::Context,
                Self::Transform,
                Self::Error,
            >,
        >,
    ) -> Result<(), Self::Error> {
        // given the scope of the Ant, we expect the seeable portion of the
        // environment to be just the tile where the Ant is currently located
        let neighborhood = neighborhood.expect("Unexpected neighborhood");
        let tile = neighborhood.center();
        // the tile in question can either be BLACK or WHITE, we encode this
        // information with a Cell entity or no entity respectively
        let entities = tile.entities();
        let black_cell =
            entities.iter().find(|e| e.borrow().kind() == Kind::Cell);

        if let Some(cell) = black_cell {
            // if the cell is BLACK, we flip its color by "killing" the entity
            // reducing its lifespan to 0 and move left
            let mut c = cell.borrow_mut();
            debug_assert_eq!(c.location(), self.location());
            let lifespan = c.lifespan_mut().expect("Invalid Cell lifespan");
            lifespan.clear();

            self.turn_left_and_move_forward();
        } else {
            // if the cell is WHITE, we flip its color by creating a new entity
            // as offspring for the next generation, and move right
            self.offspring_id += 1;
            let black_cell = Cell::new(
                self.offspring_id,
                self.location,
                self.offspring_mesh.clone(),
            );
            self.offspring.insert(black_cell);

            self.turn_right_and_move_forward();
        }

        Ok(())
    }

    fn offspring(
        &mut self,
    ) -> Option<
        Offspring<
            'static,
            Self::Id,
            Self::Kind,
            Self::Context,
            Self::Transform,
            Self::Error,
        >,
    > {
        // release the offspring (if any) to the environment
        Some(self.offspring.drain())
    }

    fn draw(
        &self,
        ctx: &mut Self::Context,
        transform: &Self::Transform,
    ) -> Result<(), Self::Error> {
        // Draw the shape of the Ant without taking into consideration the
        // given transformation (that is always going to be equal to the Identity
        // matrix) since for the purposes of this simulation neither zoom or
        // panning are supported.
        assert_eq!(transform, &graphics::DrawParam::default());

        // the radius is equal to half the grid tiles side
        let radius = env::SIDE / 2.0;
        // coordinate in pixels of the top-left corner of the mesh
        let location = self.location.to_pixel_coords(env::SIDE);
        let location = Point2::new(location.x, location.y);
        // shift the center of the shape to the center of the tile
        let offset = location + Vector2::new(radius, radius);

        graphics::draw(ctx, &self.mesh, transform.dest(offset))
    }
}
