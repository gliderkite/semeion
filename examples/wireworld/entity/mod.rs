pub mod cell;

pub use cell::*;

/// The entities Kinds.
/// The order of the kind determines the entities drawing order.
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Kind {
    Cell,
}
