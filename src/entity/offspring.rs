use std::cell::RefCell;
use std::rc::Rc;

use super::*;

/// The Entity offspring, represented as a list of heap allocated Entity traits.
#[derive(Debug)]
pub struct Offspring<'e, I, K, C, T, E> {
    entities: Vec<EntityStrongRef<'e, I, K, C, T, E>>,
}

impl<'e, I, K, C, T, E> Default for Offspring<'e, I, K, C, T, E> {
    /// Gets an empty Offspring.
    fn default() -> Self {
        Self {
            entities: Vec::default(),
        }
    }
}

impl<'e, I: Eq + Hash + Clone + Debug, K, C, T, E>
    Offspring<'e, I, K, C, T, E>
{
    /// Adds a new Entity to the Offspring.
    pub fn insert<Q>(&mut self, entity: Q)
    where
        Q: Entity<'e, Id = I, Kind = K, Context = C, Transform = T, Error = E>
            + 'e,
    {
        self.entities.push(Rc::new(RefCell::new(entity)));
    }

    /// Takes the entities out of Self to create a new Offspring. Useful when
    /// you want to release a new Entity offspring into the Environment.
    pub fn drain(&mut self) -> Self {
        Self {
            entities: self.entities.drain(..).collect(),
        }
    }

    /// Takes the entities out of the Offspring.
    pub(crate) fn take_entities(
        self,
    ) -> Vec<EntityStrongRef<'e, I, K, C, T, E>> {
        self.entities
    }
}
