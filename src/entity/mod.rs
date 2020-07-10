use std::fmt::Debug;

use super::*;

pub use lifespan::*;
pub use offspring::*;
pub use state::*;

pub mod lifespan;
pub mod offspring;
pub mod state;

/// The type of the Entity unique ID.
///
/// It is safe to assume that the library will not be dealing with a number of
/// entities greater than `usize::max_value()` at any given time.
pub type Id = usize;

/// The Trait that describes a generic Entity.
///
/// This is the Trait that defines the shared behavior for all the entities that
/// belong to the Environment. Each of the entities needs to implement this
/// Trait, and can interact with other entities via this Trait.
///
/// The lifetime `'e` is used to specify the lifetime bound of any immutable
/// reference that belongs to the type that implements this trait.
/// So that, if your type includes immutable references with an explicit
/// lifetime, it is possible to propagate the lifetime bound to the Offspring of
/// this Entity without requiring a `'static` lifetime.
/// This lifetime bound does not apply to mutable references, since they cannot
/// be copied without violate uniqueness.
pub trait Entity<'e>: Debug {
    /// The type of the Entity kind.
    type Kind;

    /// The type of the graphics Context used to draw the shape of the entities.
    type Context;

    /// Gets the ID of the Entity.
    ///
    /// The ID must be unique for all the entities. It is considered a logic
    /// error for to different entities to share the same ID, in which case the
    /// behavior within the Environment is undefined.
    fn id(&self) -> Id;

    /// Gets the Entity type.
    ///
    /// Each Entity can belong to a specific kind that defines, besides the
    /// logical type of the Entity itself, the order in which entities are going
    /// to be drawn.
    fn kind(&self) -> Self::Kind;

    /// Gets the location of the Entity within the Environment.
    ///
    /// If an Entity has no location, it should return None. An Entity can either
    /// have a location for its entire lifetime or no location; it is considered
    /// a logic error if this method returns None for an Entity that previously
    /// had a location, and vice versa.
    fn location(&self) -> Option<Location> {
        None
    }

    /// Gets the scope of this Entity.
    ///
    /// The size of the scope defines its radius of influence, i.e. the portion
    /// of the Environment that an Entity can see and interact with. The bigger
    /// the scope the bigger the portion of the Environment (Neighborhood).
    /// A scope equal to 0 means that the Entity is only going to be able to see
    /// the Tile where it currently resides, a scope equal to 1 will also include
    /// the 8 surrounding tiles, and so on.
    /// If None is returned the Entity has no scope at all, and it can neither see
    /// nor affect any other tile or surrounding Entity.
    /// In other terms, the scope effectively represents the distance from the
    /// Tile where the Entity is located, to the farthest Tile it will ever be
    /// able to reach.
    /// Moreover, only entities that have a location in the Environment can
    /// interact with surrounding entities, therefore it is a logic error to
    /// return Some from this method if `Entity::location()` returns None, but it
    /// is perfectly valid for entities to have a location but no scope.
    fn scope(&self) -> Option<Scope> {
        None
    }

    /// Gets the remaining lifespan of the Entity.
    ///
    /// If the concept of lifespan is meaningless for this Entity, it should
    /// simply return None.
    fn lifespan(&self) -> Option<Lifespan> {
        None
    }

    /// Gets a mutable reference to the remaining lifespan of the Entity.
    ///
    /// It is possible to influence the remaining lifespan of the Entity by
    /// changing its value. If the Entity has no lifespan, or it does not allow
    /// other entities to affect its own lifespan, None should be returned.
    fn lifespan_mut(&mut self) -> Option<&mut Lifespan> {
        None
    }

    /// Gets a reference to a trait that is implemented by the object that
    /// represents the state of the Entity.
    ///
    /// The State trait exposes method that enable dynamic typing and allow to
    /// downcast the trait to the original concrete type.
    /// If the Entity has no meaningful state associated with it, this method
    /// should simply return None.
    fn state(&self) -> Option<&dyn State> {
        None
    }

    /// Gets a mutable reference to a trait that is implemented by the object that
    /// represents the state of the Entity.
    ///
    /// The State trait exposes method that enable dynamic typing and allow to
    /// downcast the trait to the original concrete type.
    /// If the Entity has no meaningful state associated with it, this method
    /// should simply return None.
    fn state_mut(&mut self) -> Option<&mut dyn State> {
        None
    }

    /// Allows the Entity to observe the portion of surrounding Environment seen
    /// by the Entity according to its scope.
    ///
    /// The larger the scope the bigger the portion of the Environment that the
    /// Entity will be allowed to see.
    /// The provided Neighborhood represents the squared grid of surrounding
    /// cells. Each of these cells can be queried to detect what other entities
    /// are currently in that location, and allows to interact with those
    /// entities via the methods provided by this trait.
    /// This method is called for each generation, and the provided Neighborhood
    /// represents a snapshot of the previous generation readonly fields, that
    /// is, the Neighborhood will contain the entities according to their
    /// location in the previous generation.
    /// If the Entity has no scope, the Neighborhood will be None.
    /// If any of the operations performed in this method fails, you can bubble
    /// up the error to the Environment, which will take care of reporting it to
    /// the final user.
    ///
    /// # Note
    /// If an Entity's shared property (that is seeable or changeable
    /// by other entities, such as location or state) is modified in this method,
    /// being it a property of self or of the given neighbor entities, this will
    /// be immediately reflected when querying the other entities, but it will not
    /// immediately affect the state of the Environment itself. That is, the
    /// portion of the Neighborhood presented to other entities will still
    /// reflect the locations of the entities as it was during the previous
    /// generation, until all the entities have observed their neighborhood.
    /// For this reason, it is considered a logic error to change the shared
    /// properties here, you should instead record the changes, and apply them in
    /// the `Entity::react` method, that is guaranteed to be called for all the
    /// entities, only after all the `Entity::observe` have been called.
    fn observe(
        &mut self,
        _: Option<Neighborhood<'_, 'e, Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Allows to take an action that will affect the Entity itself, and its
    /// neighbors, according to the portion of surrounding Environment seen by
    /// the Entity according to its scope.
    ///
    /// The larger the scope the bigger the portion of the Environment the Entity
    /// will be allowed to see and affect.
    /// The provided Neighborhood represents the squared grid of surrounding
    /// cells. Each of this cell can be queried to detect what other entities
    /// are currently in that location, and allows to interact with those
    /// entities via the methods provided by this trait.
    /// This method is called for each generation, and the provided Neighborhood
    /// represents a snapshot of the previous generation readonly fields, that
    /// is, the Neighborhood will contain the entities according to their
    /// location in the previous generation.
    /// If the Entity has no scope the Neighborhood will be None.
    /// If any of the operations performed in this method fails, you can bauble
    /// up the error to the Environment, that will take care of reporting it to
    /// the final user.
    ///
    /// # Note
    /// The same semantic that applies to the `Entity::observe` method applies
    /// also to the `Entity::react` method. Therefore, changes to the Entity's
    /// shared properties such as location or state, will be immediately
    /// reflected when querying neighbors or following entities, while the given
    /// Neighborhood is guaranteed to provide a snapshot of the locations of the
    /// entities of the previous generation.
    fn react(
        &mut self,
        _: Option<Neighborhood<'_, 'e, Self::Kind, Self::Context>>,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Gets the Offspring of the Entity.
    ///
    /// The offspring of an Entity will be introduced in the Environment at
    /// every generation. Therefore, the list of entities returned by this method
    /// will be taken as is and introduces as is in the Environment. It is the
    /// responsibility of the Entity owner to return an offspring only if the
    /// Entity did actually generate an offspring in the current generation,
    /// otherwise this method should return None.
    ///
    /// The lifetime of the Entity `'e` will be propagated to its Offspring, so
    /// that the lifetime bound stipulated when creating the parent Entity, that
    /// may contain references as part of the type that implements this trait,
    /// will be kept unchanged.
    fn offspring(
        &mut self,
    ) -> Option<Offspring<'e, Self::Kind, Self::Context>> {
        None
    }

    /// Draws the Entity using the given graphics Context and according to the
    /// given transformation (matrix).
    ///
    /// This method is called for each generation. If you wish to skip drawing
    /// the shape of your Entity, this method should simply return `Ok(())`.
    fn draw(&self, _: &mut Self::Context, _: Transform) -> Result<(), Error> {
        Ok(())
    }
}

/// The Entity Trait type alias.
pub(crate) type Trait<'e, K, C> = dyn Entity<'e, Kind = K, Context = C> + 'e;
