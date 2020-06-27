use std::fmt::Debug;

use super::*;

pub use lifespan::*;
pub use offspring::*;
pub use state::*;

pub mod lifespan;
pub mod offspring;
pub mod state;

/// The Entity Trait type alias.
pub(crate) type Trait<'e, I, K, C, T, E> = dyn Entity<'e, Id = I, Kind = K, Context = C, Transform = T, Error = E>
    + 'e;

/// The Trait that describes a generic Entity.
/// This is the Trait that defines the shared behavior for all the entities that
/// belongs to the Environment. Each of the entities needs to implement this
/// Trait, and can interact with other entities via this Trait.
///
/// The lifetime `'e` is used to specify the lifetime bound of any immutable
/// reference that is part of the type that is going to implement this trait.
/// So that if your type includes an immutable reference with an explicit
/// lifetime, it is possible to propagate the lifetime bound to the Offspring of
/// this Entity without requiring a `'static` lifetime. If your object doesn't
/// include references, you should specify a `'static` lifetime for the Entity
/// and its Offspring (implying that that the object can outlive any lifetime).
/// This lifetime bound does not apply to mutable references, since they cannot
/// be copied without violate uniqueness (but can only be moved).
pub trait Entity<'e>: Debug {
    /// The type of the Entity ID. This type is ideally very cheap to Clone.
    type Id;

    /// The type of the Entity kind.
    type Kind;

    /// The type of the graphics Context used to draw the shape of the entities.
    type Context;

    /// The type of the transformation matrix (or possibly any other metadata
    /// associated with drawing operations) that is passed to the `Entity::draw()`
    /// method.
    type Transform;

    /// The type of the error returned by the Entity methods if something went
    /// wrong.
    type Error;

    /// Gets the ID of the Entity.
    /// The ID must be unique for all the entities. It is considered a logic
    /// error for to different entities to share the same ID, in which case the
    /// behavior within the environment is undefined.
    fn id(&self) -> &Self::Id;

    /// Gets the Entity type.
    /// Each Entity can belong to a specific kind that defines, besides the
    /// logical type of the Entity itself, the order in which entities are going
    /// to be drawn.
    fn kind(&self) -> Self::Kind;

    /// Gets the location of the Entity within the environment.
    /// If the Entity has no location, it should return None. An Entity can either
    /// have a location for its entire lifetime or no location; it is considered
    /// a logic error if this method returns None for an Entity that previously
    /// had a location, and vice versa.
    fn location(&self) -> Option<Location> {
        None
    }

    /// Gets the scope of this Entity. The size of the scope defines its radius
    /// of influence, that is the portion of the environment that an Entity can
    /// see and interact with. The bigger the scope the bigger the portion of
    /// the Environment (NeighborHood). A scope equal to 0 means that the Entity
    /// is only going to be able to see the tile where it currently resides, a
    /// scope equal to 1 will also include the 8 surrounding tiles, and so on.
    /// If None is returned the Entity has no scope at all, and can neither seen
    /// of affect any other tile or surrounding Entity. In other terms, the scope
    /// effectively represents the distance from the tile where the Entity is
    /// located, to the farthest tile it will ever be able to reach.
    /// Moreover, only entities that have a location in the environment can
    /// interact with surrounding entities, therefore it is a logic error to
    /// return Some from this method if `Entity::location()` returns None, but it
    /// is perfectly valid for entities to have a location but no scope.
    fn scope(&self) -> Option<Scope> {
        None
    }

    /// Gets the remaining lifespan of the Entity. If the Entity has no lifespan
    /// attached to it, because meaningless, it should return None.
    fn lifespan(&self) -> Option<Lifespan> {
        None
    }

    /// Gets a mutable reference to the remaining lifespan of the Entity.
    /// It is possible to influence the remaining lifespan of the Entity by
    /// changing its value. If the Entity has no lifespan, or it does not allow
    /// other entities to affect its own lifespan, None should be returned.
    fn lifespan_mut(&mut self) -> Option<&mut Lifespan> {
        None
    }

    /// Gets a reference to a trait that is implemented by the object that
    /// represents the state of the Entity. The State trait exposes method that
    /// enable dynamic typing and allow to downcast the trait to the original
    /// concrete type. If the Entity has no meaningful state associated with it,
    /// this method should simply return None.
    fn state(&self) -> Option<&dyn State> {
        None
    }

    /// Gets a mutable reference to a trait that is implemented by the object that
    /// represents the state of the Entity. The State trait exposes method that
    /// enable dynamic typing and allow to downcast the trait to the original
    /// concrete type. If the Entity has no meaningful state associated with it,
    /// or it does not allow other entities to affect its own state, this method
    /// should return None.
    fn state_mut(&mut self) -> Option<&mut dyn State> {
        None
    }

    /// Allows the Entity to observe the portion of surrounding environment seen
    /// by the Entity according to its scope. The larger the scope the bigger the
    /// portion of the environment that the Entity will be allowed to see.
    /// The provided NeighborHood represents the squared grid of surrounding
    /// cells. Each of these cells can be queried to detect what other entities
    /// are currently in that location, and allows to interact with those
    /// entities via the methods provided by this trait.
    /// This method is called for each generation, and the provided NeighborHood
    /// represents a snapshot of the previous generation readonly fields, that
    /// is, the NeighborHood will contain the entities according to their
    /// location in the previous generation.
    /// If the Entity has no scope, the NeighborHood will be None.
    /// If any of the operations performed in this method fails, you can bubble
    /// up the error to the Environment, which will take care of reporting it to
    /// the final user.
    ///
    /// # Note
    /// If an Entity's shared property (that is seeable or changeable
    /// by other entities, such as location or state) is modified in this method,
    /// being it a property of self or of given neighbor entities, this will be
    /// immediately reflected when querying the other entities, but it will not
    /// affect the state of the Environment itself, that is the portion of the
    /// NeighborHood presented to other entities will still reflect the locations
    /// of the entities as it was during the previous generation, until all the
    /// entities have observed their neighborhood.
    /// For this reason, it is considered a logic error to change the shared
    /// properties here, you should instead record the changes and apply them in
    /// the `Entity::react` method, that is guaranteed to be called for all the
    /// entities, only after all the `Entity::observe` have been called.
    /// You don't need to implement this method if your only requirement is to
    /// immediately react to the Entity neighborhood according to the neighbors
    /// entities locations in their previous generation, as provided by the
    /// NeighborHood (input to this method).
    fn observe(
        &mut self,
        _: Option<
            NeighborHood<
                '_,
                'e,
                Self::Id,
                Self::Kind,
                Self::Context,
                Self::Transform,
                Self::Error,
            >,
        >,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Allows to take an action that will affect the Entity itself, and its
    /// neighbors, according to the portion of surrounding environment seen by
    /// the Entity according to its scope. The larger the scope the bigger the
    /// portion of the environment the Entity will be allowed to see and affect.
    /// The provided NeighborHood represents the squared grid of surrounding
    /// cells. Each of this cell can be queried to detect what other entities
    /// are currently in that location, and allows to interact with those
    /// entities via the methods provided by this trait.
    /// This method is called for each generation, and the provided NeighborHood
    /// represents a snapshot of the previous generation readonly fields, that
    /// is, the NeighborHood will contain the entities according to their
    /// location in the previous generation.
    /// If the Entity has no scope the NeighborHood will be None.
    /// If any of the operations performed in this method fails, you can bauble
    /// up the error to the Environment, that will take care of reporting it to
    /// the final user.
    ///
    /// # Note
    /// The same semantic that applies to the `Entity::observe` method applies
    /// also to the `Entity::react` method. Therefore, changes to the Entity's
    /// shared properties such as location or state, will be immediately
    /// reflected when querying neighbors or following entities, while the given
    /// NeighborHood is guaranteed to provide a snapshot of the locations of the
    /// entities of a previous generation.
    fn react(
        &mut self,
        _: Option<
            NeighborHood<
                '_,
                'e,
                Self::Id,
                Self::Kind,
                Self::Context,
                Self::Transform,
                Self::Error,
            >,
        >,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Gets the offspring of the Entity.
    /// The offspring of an Entity will be introduced in the environment at
    /// every generation. Therefore, the list of entities returned by this method
    /// will be taken as is and introduces as is in the environment. It is the
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
    ) -> Option<
        Offspring<
            'e,
            Self::Id,
            Self::Kind,
            Self::Context,
            Self::Transform,
            Self::Error,
        >,
    > {
        None
    }

    /// Draws the Entity using the given graphics Context and according to the
    /// given transformation (matrix). Returns an error of the provided type if
    /// something went wrong.
    /// This method is called for each generation. If you wish to skip drawing
    /// the shape of your Entity, this method should simply return `Ok(())`.
    fn draw(
        &self,
        _: &mut Self::Context,
        _: &Self::Transform,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
