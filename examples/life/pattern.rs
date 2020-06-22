use super::env;
use semeion::space::*;

pub struct Pattern;

impl Pattern {
    /// Constructs the Acorn pattern.
    /// https://www.conwaylife.com/wiki/Acorn
    pub fn acorn() -> Vec<Location> {
        let offsets = vec![
            Offset { x: 0, y: 2 },
            Offset { x: -1, y: 2 },
            Offset { x: 2, y: 1 },
            Offset { x: 3, y: 2 },
            Offset { x: 4, y: 2 },
            Offset { x: 5, y: 2 },
        ];
        Self::build(env::dimension().center(), offsets)
    }

    /// Build the pattern from an initial location with the given offsets from it.
    fn build(origin: Location, offsets: Vec<Offset>) -> Vec<Location> {
        let mut locations = vec![origin];
        locations.extend(offsets.iter().map(|&delta| origin + delta));
        locations
    }
}
