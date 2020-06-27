use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameError};

use super::{Id, Kind};
use crate::env;
use semeion::*;

/// Constructs a new mesh for a Cell.
pub fn mesh(ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
    let mut mesh = graphics::MeshBuilder::new();
    let bounds = graphics::Rect::new(0.0, 0.0, env::SIDE, env::SIDE);
    let color = graphics::BLACK;
    mesh.rectangle(graphics::DrawMode::fill(), bounds, color);
    mesh.build(ctx)
}

#[derive(Debug)]
pub struct Cell {
    id: Id,
    location: Location,
    lifespan: Lifespan,
    mesh: graphics::Mesh,
}

impl Cell {
    /// Constructs a new Cell with the given ID.
    pub fn new(id: Id, location: Location, mesh: graphics::Mesh) -> Self {
        Self {
            id,
            location,
            // the lifespan of a cell is immortal, until killed by the Ant
            lifespan: Lifespan::Immortal,
            mesh,
        }
    }
}

impl<'a> Entity<'a> for Cell {
    type Id = Id;
    type Kind = Kind;
    type Context = Context;
    type Transform = graphics::DrawParam;
    type Error = GameError;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn kind(&self) -> Self::Kind {
        Kind::Cell
    }

    fn location(&self) -> Option<Location> {
        Some(self.location)
    }

    fn lifespan(&self) -> Option<Lifespan> {
        Some(self.lifespan)
    }

    fn lifespan_mut(&mut self) -> Option<&mut Lifespan> {
        // the lifespan of the Cell can be affected by the Ant behavior
        Some(&mut self.lifespan)
    }

    fn draw(
        &self,
        ctx: &mut Self::Context,
        transform: &Self::Transform,
    ) -> Result<(), Self::Error> {
        // Draw the shape of the Cell without taking into consideration the
        // given transformation (that is always going to be equal to the Identity
        // matrix) since for the purposes of this simulation neither zoom or
        // panning are supported.
        debug_assert_eq!(transform, &graphics::DrawParam::default());

        // coordinate in pixels of the top-left corner of the mesh
        let offset = self.location.to_pixel_coords(env::SIDE);
        let offset = Point2::new(offset.x, offset.y);

        graphics::draw(ctx, &self.mesh, transform.dest(offset))
    }
}
