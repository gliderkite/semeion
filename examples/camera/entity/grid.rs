use ggez::{graphics, mint::Point2, Context, GameError};
use semeion::*;

use super::Kind;
use crate::env;

/// Constructs a new mesh for a Grid.
pub fn mesh(ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
    let mut mesh = graphics::MeshBuilder::new();
    let size = env::size();
    let dimension = env::dimension();
    let stroke_width = 2.0;
    let color = graphics::Color::BLACK;

    // horizontal lines
    for i in 0..=dimension.y {
        let y = i as f32 * env::SIDE;
        let points = [Point2 { x: 0.0, y }, Point2 { x: size.width, y }];
        mesh.line(&points, stroke_width, color)?;
    }
    // vertical lines
    for i in 0..=dimension.x {
        let x = i as f32 * env::SIDE;
        let points = [Point2 { x, y: 0.0 }, Point2 { x, y: size.height }];
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
        Self {
            id: rand::random(),
            mesh,
        }
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
        graphics::draw(
            ctx,
            &self.mesh,
            graphics::DrawParam::default()
                .transform(transform.to_column_matrix4()),
        )
        .map_err(Error::with_message)
    }
}
