use super::env;
use semeion::space::*;

pub struct Pattern;

impl Pattern {
    /// Constructs the Die hard pattern.
    /// https://www.conwaylife.com/wiki/Die_hard
    pub fn diehard() -> Vec<Location> {
        let offsets = vec![
            Offset { x: 1, y: 0 },
            Offset { x: 1, y: 1 },
            Offset { x: 5, y: 1 },
            Offset { x: 6, y: 1 },
            Offset { x: 7, y: 1 },
            Offset { x: 6, y: -1 },
        ];
        Self::build(env::bounds().center(), offsets)
    }

    /// Build the pattern from an initial location with the given offsets from it.
    fn build(origin: Location, offsets: Vec<Offset>) -> Vec<Location> {
        let mut locations = vec![origin];
        locations.extend(offsets.iter().map(|&delta| origin + delta));
        locations
    }
}
