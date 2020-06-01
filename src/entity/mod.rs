use std::cell;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::{Rc, Weak};

use super::*;

pub use offspring::*;

pub mod offspring;

/// The Entity Trait type alias.
pub(crate) type Trait<I, K, C, T, E> =
    dyn Entity<Id = I, Kind = K, Context = C, Transform = T, Error = E>;

/// The RefCell type alias that owns an Entity Trait.
pub(crate) type RefCell<I, K, C, T, E> = cell::RefCell<Trait<I, K, C, T, E>>;

/// Strong reference to an Entity with interior mutability.
pub(crate) type EntityStrongRef<I, K, C, T, E> =
    Rc<entity::RefCell<I, K, C, T, E>>;

/// Weak reference to an Entity with interior mutability.
pub(crate) type EntityWeakRef<I, K, C, T, E> =
    Weak<entity::RefCell<I, K, C, T, E>>;

/// The Trait that describes a generic entity.
/// This is the Trait that defines the shared behavior for all the entities that
/// belongs to the Environment. Each of the entities needs to implement this
/// Trait and can interact with other entities via this Trait.
pub trait Entity: Debug {
    /// The type of the Entity ID. This type is ideally very cheap to Clone and
    /// Copy.
    type Id: Hash + Eq + Clone + Debug;

    /// The type of the Entity kind. This type is ideally very cheap to Clone
    /// and Copy.
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

    /// Gets the ID of the entity.
    /// The ID must be unique for all the entities. It is considered a logic
    /// error for to different entities to share the same ID, in which case the
    /// behavior within the environment is undefined.
    fn id(&self) -> &Self::Id;

    /// Gets the entity type.
    /// Each entity can belong to a specific kind that defines, besides the
    /// logical type of the entity itself, the order in which entities are goind
    /// to be drawn.
    fn kind(&self) -> Self::Kind;

    /// Gets the location of the entity within the environment.
    /// If the entity has no location, it should return None. An entity can either
    /// have a location for its entire lifetime or no location; it is considered
    /// a logic error if this method returns None for an entity that previously
    /// had a location, and vice versa.
    fn location(&self) -> Option<Location>;

    /// Gets the scope of this entity. The size of the scope defines its radius
    /// of influence, that is the portion of the environment that an Entity can
    /// see and interact with. The bigger the scope the bigger the portion of
    /// the Environment (NeighborHood). A scope equal to 0 means that the Entity
    /// is only going to be able to see the tile where it currently resides, a
    /// scope equal to 1 will also include the 8 surrounding tiles, and so on.
    /// If None is returned the Entity has no scope at all, and can neither seen
    /// of affect any other tile or surrounding Entity.
    /// Moreover, only entities that have a location in the environment can
    /// interact with surrounding entities, therefore it is a logic error to
    /// return Some from this method if `Entity::location()` returns None, but it
    /// is perfectly valid for entities to have a location but no scope.
    fn scope(&self) -> Option<usize>;

    /// Gets the remaining lifespan of the entity. If the entity has no lifespan
    /// attached to it, because meaningless, it should return None.
    fn lifespan(&self) -> Option<Lifespan>;

    /// Gets a mutable reference to the remaining lifespan of the entity.
    /// It is possible to influence the remaining lifespan of the entity by
    /// changing its value. If the entity has no lifespan, or it does not allow
    /// other entities to affect its own lifespan, None should be returned.
    fn lifespan_mut(&mut self) -> Option<&mut Lifespan>;

    /// Allows to take an action that will affect the entities itself according
    /// to the portion of surrounding environment seen by the entity according
    /// to its scope. The larger the scope the bigger the portion of the
    /// environment the entity will be allowed to see and affect.
    /// The provided NeighborHood represents the squared grid of surrounding
    /// cells. Each of this cell can be queried to detect what other entities
    /// are currently in that location, and allows to interact with those
    /// entities via the methods provided by this trait.
    /// This method is called for each generation, and the provided NeighborHood
    /// represents a snapshot of the previous generation readonly fields, that
    /// is, the NeighborHood will contain the entities according to their
    /// location in the previous generation.
    /// If the Entity has no scope the NeighborHood will be None.
    fn act(
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
    );

    /// Gets the offspring of the entity.
    /// The offspring of an entity will be introduced in the environment at
    /// every generation. Therefore, the list of entities returned by this method
    /// will be taken as is and introduces as is in the environment. It is the
    /// responsibility of the entity owner to return an offspring only if the
    /// entity did actually generate an offspring in the current generation,
    /// otherwise this method should return None.
    fn offspring(
        &mut self,
    ) -> Option<
        Offspring<
            Self::Id,
            Self::Kind,
            Self::Context,
            Self::Transform,
            Self::Error,
        >,
    >;

    /// Draws the entity using the given graphics Context and according to the
    /// given transformation (matrix). Returns an error of the provided type if
    /// something went wrong.
    /// This method is called for each generation. If you wish to skip drawing
    /// the shape of your entity, this method should simply return `Ok(())`.
    fn draw(
        &self,
        context: &mut Self::Context,
        transform: &Self::Transform,
    ) -> Result<(), Self::Error>;
}
