use super::*;

/// The Entity offspring.
pub struct Offspring<'e, K, C> {
    entities: Vec<Box<entity::Trait<'e, K, C>>>,
}

impl<'e, K, C> Default for Offspring<'e, K, C> {
    /// Constructs an empty Offspring.
    fn default() -> Self {
        Self {
            entities: Vec::default(),
        }
    }
}

impl<'e, K, C> Offspring<'e, K, C> {
    /// Inserts a new Entity to the Offspring.
    pub fn insert<E>(&mut self, entity: E)
    where
        E: Entity<'e, Kind = K, Context = C> + 'e,
    {
        self.entities.push(Box::new(entity));
    }

    /// Gets the number of entities in the Offspring.
    pub fn count(&self) -> usize {
        self.entities.len()
    }

    /// Returns true only if there are no entities in the Offspring.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Takes the entities out of self to create a new Offspring.
    ///
    /// Useful when you want to release a new Entity Offspring into the
    /// Environment while resetting the state of your Offspring for the next
    /// generation.
    pub fn drain(&mut self) -> Self {
        Self {
            entities: self.entities.drain(..).collect(),
        }
    }

    /// Takes the entities out of the Offspring consuming self.
    pub(crate) fn take_entities(self) -> Vec<Box<entity::Trait<'e, K, C>>> {
        self.entities
    }
}
