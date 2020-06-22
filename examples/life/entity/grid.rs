use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameError};

use super::{Id, Kind};
use crate::env;
use semeion::*;

/// The ID of the Grid.
const ID: Id = -1;

/// Constructs a new mesh for a Grid.
pub fn mesh(ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
    let mut mesh = graphics::MeshBuilder::new();
    let size = env::size();
    let dimension = env::dimension();
    let stroke_width = 0.5;
    let color = graphics::BLACK;

    // horizontal lines
    for i in 0..=dimension.y {
        let y = i as f32 * env::SIDE;
        let points = [Point2::new(0.0, y), Point2::new(size.width, y)];
        mesh.line(&points, stroke_width, color)?;
    }
    // vertical lines
    for i in 0..=dimension.x {
        let x = i as f32 * env::SIDE;
        let points = [Point2::new(x, 0.0), Point2::new(x, size.height)];
        mesh.line(&points, stroke_width, color)?;
    }

    mesh.build(ctx)
}

#[derive(Debug)]
pub struct Grid {
    id: Id,
    mesh: graphics::Mesh,
}

impl Grid {
    /// Constructs a new grid with the same environment size.
    pub fn new(mesh: graphics::Mesh) -> Self {
        Self { id: ID, mesh }
    }
}

impl<'a> Entity<'a> for Grid {
    type Id = Id;
    type Kind = Kind;
    type Context = Context;
    type Transform = graphics::DrawParam;
    type Error = GameError;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn kind(&self) -> Self::Kind {
        Kind::Grid
    }

    fn draw(
        &self,
        context: &mut Self::Context,
        transform: &Self::Transform,
    ) -> Result<(), Self::Error> {
        debug_assert_eq!(transform, &graphics::DrawParam::default());
        graphics::draw(context, &self.mesh, *transform)
    }
}
