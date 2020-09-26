use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameError};

use super::Kind;
use crate::env;
use semeion::*;

/// Constructs a new mesh for a Grid.
pub fn mesh(ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
    let mut mesh = graphics::MeshBuilder::new();
    let size = env::size();
    let dimension = env::dimension();
    let stroke_width = 2.0;
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
    pub fn new(mesh: graphics::Mesh) -> Box<Self> {
        Box::new(Self {
            id: rand::random(),
            mesh,
        })
    }
}

impl<'a> Entity<'a> for Grid {
    type Kind = Kind;
    type Context = Context;

    fn id(&self) -> Id {
        self.id
    }

    fn kind(&self) -> Self::Kind {
        Kind::Grid
    }

    fn draw(
        &self,
        ctx: &mut Self::Context,
        transform: Transform,
    ) -> Result<(), Error> {
        graphics::push_transform(ctx, Some(transform.to_column_matrix4()));
        graphics::apply_transformations(ctx).map_err(Error::with_message)?;

        graphics::draw(ctx, &self.mesh, graphics::DrawParam::default())
            .map_err(Error::with_message)?;

        graphics::pop_transform(ctx);
        graphics::apply_transformations(ctx).map_err(Error::with_message)
    }
}
