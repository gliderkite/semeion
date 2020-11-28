use ggez::{
    graphics::{DrawMode, Mesh, MeshBuilder, Rect, WHITE},
    GameResult,
};

use crate::{env, palette::Palette};

/// The cellular automaton rule (there are only 256 possible rules).
pub type Rule = u8;

/// The entities kind. Since we only use a single kind (the Cell) this can be
/// defined as the unit type.
pub type Kind = ();

/// State shared between all the entities.
#[derive(Debug)]
pub struct Context {
    pub palette: Palette,
    pub cell_mesh: Mesh,
    pub rule: Rule,
}

impl Context {
    /// Constructs a new context.
    pub fn new(rule: Rule, ctx: &mut ggez::Context) -> GameResult<Self> {
        Ok(Self {
            palette: Palette::default(),
            cell_mesh: make_cell_mesh(ctx)?,
            rule,
        })
    }
}

/// Constructs a new mesh for a Cell.
fn make_cell_mesh(ctx: &mut ggez::Context) -> GameResult<Mesh> {
    let mut mesh = MeshBuilder::new();
    let bounds = Rect::new(0.0, 0.0, env::SIDE, env::SIDE);
    // by default the fill color is white so that it will be replaced (blended) by
    // the color retrieved from the palette according to the Cell age
    mesh.rectangle(DrawMode::fill(), bounds, WHITE);
    mesh.build(ctx)
}
