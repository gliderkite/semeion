use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameError};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Weak;

use super::{Id, Kind};
use crate::env;
use semeion::*;

/// Constructs a new mesh for a Cell.
pub fn mesh(ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
    let mut mesh = graphics::MeshBuilder::new();
    let bounds = graphics::Rect::new(0.0, 0.0, env::SIDE, env::SIDE);
    let color = graphics::BLACK;
    mesh.rectangle(graphics::DrawMode::fill(), bounds, color);
    mesh.build(ctx)
}

#[derive(Debug)]
pub struct Cell<'a> {
    id: Id,
    location: Location,
    lifespan: Lifespan,
    // all the Cells share the same Mesh
    mesh: &'a graphics::Mesh,
    offspring: Offspring<'a, Id, Kind, Context, graphics::DrawParam, GameError>,
    visited: Weak<RefCell<HashSet<Location>>>,
}

impl<'a> Cell<'a> {
    /// Constructs a new Cell with the given ID.
    pub fn new(
        location: Location,
        mesh: &'a graphics::Mesh,
        visited: Weak<RefCell<HashSet<Location>>>,
    ) -> Self {
        Self {
            // ID are simply randomly generated as the possibility of collisions
            // are very very low
            id: rand::random(),
            location,
            // the lifespan of a cell is independent on the passing of time
            // (generations), but instead it will exclusively depends on the
            // neighbors cells
            lifespan: Lifespan::Immortal,
            mesh,
            offspring: Offspring::default(),
            visited,
        }
    }
}

impl<'a> Entity<'a> for Cell<'a> {
    type Id = Id;
    type Kind = Kind;
    type Context = Context;
    type Transform = graphics::DrawParam;
    type Error = GameError;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn kind(&self) -> Self::Kind {
        Kind::Cell
    }

    fn location(&self) -> Option<Location> {
        Some(self.location)
    }

    fn scope(&self) -> Option<Scope> {
        // The scope of a Cell is the portion of the environment immediately
        // surrounding it (besides the tile where it is located). Nevertheless,
        // since we are going to implement the 2nd rule of the game (any dead cell
        // with three live neighbors becomes a live cell) without encoding dead
        // cells all over the environment as existing entities, we can expand the
        // scope of influence of this cell by 1, to be able to "spy" on the
        // neighbors of the neighbors, and see if for any neighbor that is a dead
        // cell any of its neighbors are alive.
        Some(Scope::with_magnitude(2))
    }

    fn lifespan(&self) -> Option<Lifespan> {
        Some(self.lifespan)
    }

    /// Game of Life rules:
    /// 1. Any live cell with two or three live neighbors survives.
    /// 2. Any dead cell with three live neighbors becomes a live cell.
    /// 3. All other live cells die in the next generation. Similarly, all other
    ///     dead cells stay dead.
    fn react(
        &mut self,
        neighborhood: Option<
            NeighborHood<
                Self::Id,
                Self::Kind,
                Self::Context,
                Self::Transform,
                Self::Error,
            >,
        >,
    ) -> Result<(), Self::Error> {
        // given the scope of the Cell (= 2) we expect a valid neighborhood
        let neighborhood = neighborhood.expect("Invalid neighborhood");
        // a Scope = 1 allows to query the cells in the immediate surroundings
        let scope = Scope::with_magnitude(1);

        // any live cell with two or three live neighbors survives (otherwise it
        // dies) => count the number of entities found in all the tiles that
        // are included in this Cell immediate border
        let count: usize = neighborhood
            .border(Offset::origin(), scope)
            .expect("Invalid border")
            .iter()
            .map(|t| t.len())
            .sum();
        if !(count == 2 || count == 3) {
            // this cell will die this generation
            self.lifespan.clear();
        }

        // for each tile in this Cell (immediate) border, if that tile contains
        // a dead cell (that is, no entities are in it), then get its (immediate)
        // border and check whether there are 3 living cells, in which case the
        // dead cell becomes alive
        for offset in Offset::border(scope) {
            // the location in the environment of the tile belonging to this
            // Cell border
            let loc =
                *self.location.clone().translate(offset, env::dimension());

            // skip this tile if already visited by another Cell, or store this
            // information in the cache of already visited tiles
            if let Some(visited) = self.visited.upgrade() {
                if visited.borrow().contains(&loc) {
                    continue;
                } else {
                    visited.borrow_mut().insert(loc);
                }
            }

            // skip the tile if it already contains a living cell
            if !neighborhood.tile(offset).is_empty() {
                // if there is an entity in this tile, it must be a single living
                // Cell, anything else would be an error
                assert_eq!(neighborhood.tile(offset).len(), 1);
                continue;
            }

            // gets the tiles in the border of the neighbor cell, and count how
            // many alive cells are in those
            let count: usize = neighborhood
                .border(offset, scope)
                .expect("Invalid border")
                .iter()
                .map(|t| t.len())
                .sum();
            // any dead cell with three live neighbors becomes a live cell
            if count == 3 {
                assert!(neighborhood.tile(offset).is_empty());
                // this Cell will introduce in the environment a new living cell
                // as part of its offspring
                self.offspring.insert(Cell::new(
                    loc,
                    self.mesh,
                    self.visited.clone(),
                ))
            }
        }

        Ok(())
    }

    fn offspring(
        &mut self,
    ) -> Option<
        Offspring<
            'a,
            Self::Id,
            Self::Kind,
            Self::Context,
            Self::Transform,
            Self::Error,
        >,
    > {
        // release the offspring (if any) to the environment
        Some(self.offspring.drain())
    }

    fn draw(
        &self,
        ctx: &mut Self::Context,
        transform: &Self::Transform,
    ) -> Result<(), Self::Error> {
        // Draw the shape of the Cell without taking into consideration the
        // given transformation (that is always going to be equal to the Identity
        // matrix) since for the purposes of this simulation neither zoom or
        // panning are supported.
        debug_assert_eq!(transform, &graphics::DrawParam::default());

        // coordinate in pixels of the top-left corner of the mesh
        let offset = self.location.to_pixel_coords(env::SIDE);
        let offset = Point2::new(offset.x, offset.y);

        graphics::draw(ctx, self.mesh, transform.dest(offset))
    }
}
