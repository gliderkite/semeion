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
    let bounds = env::bounds();
    let stroke_width = 0.5;
    let color = graphics::BLACK;

    // horizontal lines
    for i in 0..=bounds.y {
        let y = i as f32 * env::SIDE;
        let points = [Point2::new(0.0, y), Point2::new(size.width, y)];
        mesh.line(&points, stroke_width, color)?;
    }
    // vertical lines
    for i in 0..=bounds.x {
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

impl Entity for Grid {
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

    fn location(&self) -> Option<Location> {
        // The Grid belongs to the whole environment, it has not specific location
        // within it, and it is never contained in a single cell.
        None
    }

    fn scope(&self) -> Option<usize> {
        // The grid is a static entity, it cannot interact with any other, and
        // therefore has no scope.
        None
    }

    fn lifespan(&self) -> Option<Lifespan> {
        // The grid has no lifespan.
        None
    }

    fn lifespan_mut(&mut self) -> Option<&mut Lifespan> {
        // No other entity can affect the Grid lifespan.
        None
    }

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
    ) {
        // The Grid is not alive. It cannot take any action or influence the
        // environment, ever. There is nothing to do here.
        assert!(neighborhood.is_none());
    }

    fn offspring(
        &mut self,
    ) -> Option<
        Offspring<
            Self::Id,
            Self::Kind,
            Self::Context,
            Self::Transform,
            Self::Error,
        >,
    > {
        // The Grid is not alive. It will not produce any offspring.
        None
    }

    fn draw(
        &self,
        context: &mut Self::Context,
        transform: &Self::Transform,
    ) -> Result<(), Self::Error> {
        assert_eq!(transform, &graphics::DrawParam::default());
        graphics::draw(context, &self.mesh, *transform)
    }
}
