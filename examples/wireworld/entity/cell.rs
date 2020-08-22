use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameError};
use std::any::Any;

use super::Kind;
use crate::{env, Meshes};
use semeion::*;

/// Enumerate each possible cell state (Empty will be encoded as the absence
/// of the Cell entity in a particular location).
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum State {
    ElectronHead,
    ElectronTail,
    Conductor,
}

/// Implement the entity::State trait to allow downcasting when querying the
/// Cell state via the Entity::state() method.
impl entity::State for State {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl State {
    /// Constructs a new Mesh with a color that depends on the State of the Cell.
    pub fn mesh(self, ctx: &mut Context) -> Result<graphics::Mesh, GameError> {
        match self {
            Self::ElectronHead => {
                let blue = graphics::Color::new(0.0, 0.0, 255.0, 1.0);
                Self::build_mesh(ctx, blue)
            }
            Self::ElectronTail => {
                let red = graphics::Color::new(255.0, 0.0, 0.0, 1.0);
                Self::build_mesh(ctx, red)
            }
            Self::Conductor => {
                let yellow = graphics::Color::new(255.0, 255.0, 0.0, 1.0);
                Self::build_mesh(ctx, yellow)
            }
        }
    }

    /// Constructs a new mesh for a Cell with the given color.
    fn build_mesh(
        ctx: &mut Context,
        color: graphics::Color,
    ) -> Result<graphics::Mesh, GameError> {
        let mut mesh = graphics::MeshBuilder::new();
        let bounds = graphics::Rect::new(0.0, 0.0, env::SIDE, env::SIDE);
        mesh.rectangle(graphics::DrawMode::fill(), bounds, color);
        mesh.build(ctx)
    }
}

/// Represents the current and the following (generation) state of the Cell.
#[derive(Debug)]
struct StateSnapshot {
    current: State,
    next: State,
}

impl StateSnapshot {
    /// Constructs a new StateSnapshot where the current state and the following
    /// state are equal to the given one.
    fn new(state: State) -> Self {
        Self {
            current: state,
            next: state,
        }
    }
}

#[derive(Debug)]
pub struct Cell<'a> {
    id: Id,
    location: Location,
    meshes: &'a Meshes,
    state: StateSnapshot,
}

impl<'a> Cell<'a> {
    /// Constructs a new Cell with the given ID.
    pub fn new(location: Location, state: State, meshes: &'a Meshes) -> Self {
        Self {
            // ID are simply randomly generated as the possibility of collisions
            // are very very low
            id: rand::random(),
            location,
            meshes,
            state: StateSnapshot::new(state),
        }
    }
}

impl<'a> Entity<'a> for Cell<'a> {
    type Kind = Kind;
    type Context = Context;

    fn id(&self) -> Id {
        self.id
    }

    fn kind(&self) -> Self::Kind {
        Kind::Cell
    }

    fn location(&self) -> Option<Location> {
        Some(self.location)
    }

    fn scope(&self) -> Option<Scope> {
        // each cell can only see the immediate surroundings
        Some(Scope::with_magnitude(1))
    }

    fn state(&self) -> Option<&dyn entity::State> {
        // returns the current state of this Cell
        Some(&self.state.current)
    }

    /// Wireworld rules:
    /// - Empty → Empty,
    /// - Electron head → Electron tail,
    /// - Electron tail → Conductor,
    /// - Conductor → Electron head if exactly one or two of the neighboring
    ///     cells are electron heads, otherwise remains conductor.
    fn observe(
        &mut self,
        neighborhood: Option<Neighborhood<Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        self.state.next = match self.state.current {
            State::ElectronHead => State::ElectronTail,
            State::ElectronTail => State::Conductor,
            State::Conductor => {
                // count the number of surrounding cells that are in the electron
                // head state
                let hood = neighborhood.expect("Invalid neighborhood");
                let border = hood
                    .immediate_border(Scope::with_magnitude(1))
                    .expect("Invalid border");
                let neighbors = border.iter().map(|t| t.entities()).flatten();

                // count the number of neighbors that are electron heads
                let count = neighbors
                    .filter(|e| {
                        let state = e
                            .state()
                            .and_then(|s| s.as_any().downcast_ref::<State>())
                            .expect("Invalid state");
                        state == &State::ElectronHead
                    })
                    .count();

                if count == 1 || count == 2 {
                    State::ElectronHead
                } else {
                    State::Conductor
                }
            }
        };

        Ok(())
    }

    fn react(
        &mut self,
        _: Option<Neighborhood<Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        // update the state of the Cell according to what was previously observed
        self.state.current = self.state.next;
        Ok(())
    }

    fn draw(
        &self,
        ctx: &mut Self::Context,
        transform: Transform,
    ) -> Result<(), Error> {
        // Draw the shape of the Cell without taking into consideration the
        // given transformation (that is always going to be equal to the Identity
        // matrix) since for the purposes of this simulation neither zoom or
        // panning are supported.
        debug_assert_eq!(transform, Transform::identity());

        // coordinate in pixels of the top-left corner of the mesh
        let offset = self.location.to_pixel_coords(env::SIDE);
        let offset = Point2::new(offset.x, offset.y);

        let mesh = self
            .meshes
            .get(self.state.current)
            .unwrap_or_else(|| panic!("No mesh for state {:?}", self.state));

        let param = graphics::DrawParam::default();
        graphics::draw(ctx, mesh, param.dest(offset))
            .map_err(Error::with_message)
    }
}
