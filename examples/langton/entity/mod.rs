pub mod ant;
pub mod cell;
pub mod grid;

pub use ant::*;
pub use cell::*;
pub use grid::*;

/// The type used for the entities IDs.
/// Negative numbers are used for the Grid and the Ant, while positive numbers
/// are used for the Cells.
pub type Id = i32;

/// The entities Kinds.
/// The order of the kind determines the entities drawing order.
#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub enum Kind {
    Grid,
    Cell,
    Ant,
}
