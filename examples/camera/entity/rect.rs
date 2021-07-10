use ggez::{graphics, Context, GameError};

use super::Kind;
use crate::env;
use semeion::*;

/// The size of the shape of a Rect entity (that can differ from the size of the
/// environment Tile in which it's located).
const ENTITY_SIZE: f32 = env::SIDE + 10.0;

/// Constructs a new mesh for the Rect.
pub fn mesh(ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
    let mut mesh = graphics::MeshBuilder::new();
    let bounds = graphics::Rect::new(0.0, 0.0, ENTITY_SIZE, ENTITY_SIZE);
    let color = graphics::Color::new(1.0, 0.0, 0.0, 1.0);
    mesh.rectangle(graphics::DrawMode::fill(), bounds, color)?;
    mesh.build(ctx)
}

#[derive(Debug)]
pub struct Rect {
    id: Id,
    location: Location,
    angle: f32,
    mesh: graphics::Mesh,
}

impl Rect {
    /// Constructs a new Rect, with an initial location within the environment.
    pub fn new(location: Location, mesh: graphics::Mesh) -> Self {
        Self {
            id: rand::random(),
            location,
            angle: 0.0,
            mesh,
        }
    }
}

impl<'a> Entity<'a> for Rect {
    type Kind = Kind;
    type Context = Context;

    fn id(&self) -> Id {
        self.id
    }

    fn kind(&self) -> Self::Kind {
        Kind::Rect
    }

    fn react(
        &mut self,
        _: Option<Neighborhood<Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        // increase the angle to make the Rect rotate around its center at each
        // generation
        self.angle += 10.0;
        Ok(())
    }

    fn draw(
        &self,
        ctx: &mut Self::Context,
        mut transform: Transform,
    ) -> Result<(), Error> {
        // shift the center of the Rect to the center of the Tile
        let half_size = ENTITY_SIZE / 2.0;
        let center_offset = half_size - env::SIDE / 2.0;
        let loc = self.location.to_pixel_coords(env::SIDE) - center_offset;

        // translate according to the entity location and rotate around its center
        transform *= Transform::translate(loc)
            * Transform::rotate_around(self.angle, [half_size, half_size]);

        graphics::draw(
            ctx,
            &self.mesh,
            graphics::DrawParam::default()
                .transform(transform.to_column_matrix4()),
        )
        .map_err(Error::with_message)
    }
}
