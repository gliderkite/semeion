use super::{cell, env};
use semeion::space::*;

pub struct Pattern;

impl Pattern {
    /// Constructs the Double Clock pattern.
    pub fn clock() -> Vec<(Location, cell::State)> {
        let mut offsets = vec![];

        for &y in &[-1, 1] {
            for x in 0..4 {
                offsets.push(Offset { x, y });
            }
        }
        for i in 3..14 {
            offsets.push(Offset { x: i, y: 0 });
        }
        offsets.push(Offset { x: 1, y: -2 });
        offsets.push(Offset { x: 1, y: 2 });
        for &y in &[-3, 3] {
            for x in -5..1 {
                offsets.push(Offset { x, y });
            }
        }
        for &y in &[-4, -2, 2, 4] {
            for x in -13..-5 {
                offsets.push(Offset { x, y });
            }
        }
        offsets.push(Offset { x: -14, y: -3 });
        offsets.push(Offset { x: -14, y: 3 });

        let origin = env::dimension().center();
        let mut cells = Vec::with_capacity(offsets.len() + 1);
        cells.push((origin, cell::State::Conductor));
        cells.extend(offsets.iter().map(|&delta| {
            let state = if delta == (Offset { x: -7, y: -2 })
                || delta == (Offset { x: -8, y: 2 })
            {
                cell::State::ElectronTail
            } else if delta == (Offset { x: -8, y: -2 })
                || delta == (Offset { x: -9, y: 2 })
            {
                cell::State::ElectronHead
            } else {
                cell::State::Conductor
            };
            (origin + delta, state)
        }));
        cells
    }
}
