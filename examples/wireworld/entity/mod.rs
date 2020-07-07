pub mod cell;

pub use cell::*;

/// The entities Kinds.
/// The order of the kind determines the entities drawing order.
#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
pub enum Kind {
    Cell,
}
