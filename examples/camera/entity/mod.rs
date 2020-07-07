pub mod grid;
pub mod rect;

pub use grid::*;
pub use rect::*;

/// The entities Kinds.
/// The order of the kind determines the entities drawing order.
#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord, Copy, Clone)]
pub enum Kind {
    Grid,
    Rect,
}
