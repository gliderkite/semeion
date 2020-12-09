use super::*;

/// The Entity offspring.
#[derive(Debug)]
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
    /// Constructs an empty Offspring with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: Vec::with_capacity(capacity),
        }
    }

    /// Inserts a new Entity into the Offspring.
    #[cfg(not(feature = "parallel"))]
    pub fn insert<E>(&mut self, entity: E)
    where
        // Trait aliases https://github.com/rust-lang/rust/issues/41517
        E: Entity<'e, Kind = K, Context = C> + 'e,
    {
        self.entities.push(Box::new(entity));
    }

    /// Inserts a new Entity into the Offspring.
    #[cfg(feature = "parallel")]
    pub fn insert<E>(&mut self, entity: E)
    where
        // Trait aliases https://github.com/rust-lang/rust/issues/41517
        E: Entity<'e, Kind = K, Context = C> + 'e + Send + Sync,
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
