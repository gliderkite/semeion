use ggez::{graphics, mint::Point2};
use semeion::*;
use std::any::Any;
use std::rc::Rc;

use crate::{
    context::{Context, Kind},
    env,
};

#[derive(Debug)]
pub struct Cell<'a> {
    id: Id,
    location: Location,
    lifespan: Lifespan,
    state: State,
    is_frozen: bool,
    age: u64,
    offspring: Offspring<'a, Kind, ggez::Context>,
    context: Rc<Context>,
}

impl<'a> Cell<'a> {
    /// Constructs a new Cell.
    pub fn new(
        location: impl Into<Location>,
        state: State,
        context: Rc<Context>,
    ) -> Self {
        Self {
            // IDs are simply randomly generated as the possibility of collisions
            // are very very low
            id: rand::random(),
            location: location.into(),
            // a Cell can live at most a number of generations equal to the height
            // of the environment, so that it will be removed before being replaced
            // once the loop is complete
            lifespan: Lifespan::with_span(env::dimension().y as u64 - 1),
            state,
            // a Cell is frozen only if its state cannot be changed anymore
            is_frozen: false,
            // the number of generations this cell has been alive, the age will
            // be used to determine the color of the cell
            age: 0,
            // a cell will generate a single offspring, representing itself in a
            // new state
            offspring: Offspring::with_capacity(1),
            context,
        }
    }

    /// Gets the new state of this Cell according to its left and right neighbors,
    /// as well as the Rule to apply.
    fn next_state(&self, left: State, right: State) -> State {
        // Gets the state of the bit in the given position.
        let bit_at = |pos: u8| -> State {
            if pos < 32 {
                (self.context.rule & (1 << pos) != 0).into()
            } else {
                panic!("invalid bit position {}", pos);
            }
        };

        match (left, self.state, right) {
            (State::Alive, State::Alive, State::Alive) => bit_at(7),
            (State::Alive, State::Alive, State::Dead) => bit_at(6),
            (State::Alive, State::Dead, State::Alive) => bit_at(5),
            (State::Alive, State::Dead, State::Dead) => bit_at(4),
            (State::Dead, State::Alive, State::Alive) => bit_at(3),
            (State::Dead, State::Alive, State::Dead) => bit_at(2),
            (State::Dead, State::Dead, State::Alive) => bit_at(1),
            (State::Dead, State::Dead, State::Dead) => bit_at(0),
        }
    }
}

impl<'a> Entity<'a> for Cell<'a> {
    type Kind = Kind;
    type Context = ggez::Context;

    fn id(&self) -> Id {
        self.id
    }

    fn kind(&self) -> Self::Kind {
        // the only Kind is the Cell, there's no need to return anything more
        // meaningful than the unit type
    }

    fn location(&self) -> Option<Location> {
        Some(self.location)
    }

    fn scope(&self) -> Option<Scope> {
        // The scope of a Cell is the portion of the environment immediately
        // surrounding it (besides the tile where it is located).
        Some(Scope::with_magnitude(1))
    }

    fn lifespan(&self) -> Option<Lifespan> {
        Some(self.lifespan)
    }

    fn state(&self) -> Option<&dyn entity::State> {
        Some(&self.state)
    }

    fn react(
        &mut self,
        neighborhood: Option<Neighborhood<Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        // each generation the Cell will age by a single unit of time
        self.lifespan.shorten();

        if self.is_frozen {
            if self.state == State::Dead {
                // this cell is frozen and dead, we can remove it by clearing its
                // remaining lifetime
                self.lifespan.clear();
            }
            // no further action needs to be taken for frozen cells
            return Ok(());
        }

        let neighborhood = neighborhood.expect("invalid neighborhood");

        // gets the state of a cell in the given position relative to this one
        let get_state_at = |offset: Offset| {
            let entities = neighborhood.tile(offset);
            // we expect one cell in any neighbor (left/right) tile at any given time
            debug_assert_eq!(entities.count(), 1);
            let entity = entities.entities().next().expect("cell not found");
            let state = entity
                .state()
                .and_then(|s| s.as_any().downcast_ref::<State>())
                .expect("invalid state");
            *state
        };

        let left_state = get_state_at(Offset { x: -1, y: 0 });
        let right_state = get_state_at(Offset { x: 1, y: 0 });
        let next_state = self.next_state(left_state, right_state);

        // create a new cell just below this one with a state that represents the
        // state this cell will have in the following generation
        let below = *self.location.clone().translate((0, 1), env::dimension());
        let mut child = Self::new(below, next_state, Rc::clone(&self.context));
        if next_state == State::Alive {
            child.age = self.age + 1;
        }
        self.offspring.insert(child);

        // freeze this cell in its current state
        self.is_frozen = true;

        Ok(())
    }

    fn offspring(
        &mut self,
    ) -> Option<Offspring<'a, Self::Kind, Self::Context>> {
        // release the offspring to the environment
        debug_assert!(self.offspring.count() <= 1);
        Some(self.offspring.drain())
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

        if self.state == State::Dead {
            // dead cells won't be drawn
            return Ok(());
        }

        // coordinate in pixels of the top-left corner of the mesh
        let offset = self.location.to_pixel_coords(env::SIDE);
        let offset = Point2 {
            x: offset.x,
            y: offset.y,
        };

        // get a new color according to the Cell age
        let color = self.context.palette.get(self.age);
        let param = graphics::DrawParam::default().color(color);

        graphics::draw(ctx, &self.context.cell_mesh, param.dest(offset))
            .map_err(Error::with_message)
    }
}

/// The state of a cell at any given time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

impl From<bool> for State {
    fn from(is_alive: bool) -> Self {
        if is_alive {
            Self::Alive
        } else {
            Self::Dead
        }
    }
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
