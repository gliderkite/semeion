pub mod grid;
pub mod rect;

pub use grid::*;
pub use rect::*;

/// The entities Kinds.
/// The order of the kind determines the entities drawing order.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Kind {
    Grid,
    Rect,
}
