use std::collections::{BTreeMap, HashMap};

use super::*;
use tile::*;

mod neighborhood;
mod tile;

#[cfg(feature = "parallel")]
mod scheduler;

pub use neighborhood::*;
pub use tile::TileView;

/// Unordered map of entities identified by their IDs, where all the entities
/// belongs to the same Kind.
type Entities<'e, K, C> = Vec<Box<entity::Trait<'e, K, C>>>;

/// Sorted map of all the entities by Kind.
type EntitiesKinds<'e, K, C> = BTreeMap<K, Entities<'e, K, C>>;

/// The Environment is a grid, of squared tiles with the same size, where all
/// the entities belong.
///
/// The Environment acts both as the data structure as well as the engine that
/// gives life to all the entities in it, and allows their interaction for every
/// generation. Where the behavior of each Entity is defined by the user, via
/// the Entity trait.
///
/// Once the Environment is initialized by inserting entities as its initial
/// population, it can be drawn by drawing all its entities, and it is possible
/// to move to the next generation (allowing the interaction between the entities
/// to take place).
///
/// An Environment can contains entities of different kinds, and it can be
/// created with specific dimension, that represents the size of the grid that
/// describes its geometry.
/// The geometry of the Environment is defined as a Torus, that is, the grid
/// dimension are adjacent to each other, allowing therefore the entities to move
/// past each dimension into the next tile as if there were no limits.
///
/// The lifetime `'e` is the lifetime bound that is applied to all the entities
/// owned by the Environment, and it must be the same lifetime for all the
/// entities types that implement the Entity trait. This lifetime defines the
/// bound for the objects (immutable references lifetimes) that implement the
/// Entity trait, and it allows to propagate the same bound to the entities
/// Offspring.
#[derive(Debug)]
pub struct Environment<'e, K, C> {
    // the list of strong references to the entities
    entities: EntitiesKinds<'e, K, C>,
    // the (1-dimensional) grid of tiles that stores week references to the
    // entities according to their location
    tiles: Tiles<'e, K, C>,
    // the latest snapshot of the environment, used to update the entities
    // properties within it at each generation
    snapshots: Vec<Snapshot<K>>,
    // the generation counter
    generation: u64,
    #[cfg(feature = "parallel")]
    scheduler: scheduler::Scheduler,
}

#[derive(Debug)]
struct Snapshot<K> {
    id: Id,
    kind: K,
    location: Location,
}

impl<'e, K: Ord, C> Environment<'e, K, C> {
    /// Constructs a new environment with the given dimension.
    ///
    /// The dimension represents the size of the grid of squared tiles of same
    /// side length, as number of columns and rows.
    pub fn new(dimension: impl Into<Dimension>) -> Self {
        let dimension = dimension.into();
        Self {
            entities: BTreeMap::new(),
            tiles: Tiles::new(dimension),
            snapshots: Vec::default(),
            generation: 0,
            #[cfg(feature = "parallel")]
            scheduler: scheduler::Scheduler::new(
                dimension,
                rayon::current_num_threads(),
            ),
        }
    }

    /// Gets the Dimension of the Environment.
    pub fn dimension(&self) -> Dimension {
        self.tiles.dimension()
    }

    /// Inserts the given Entity into the Environment.
    ///
    /// This method is usually used to pre-populate the environment with a set
    /// of entities that will constitute the first generation. After the
    /// environment has been pre-populated the set of entities stored in it will
    /// depend on the behavior of the entities itself (such ad lifespan increase
    /// and decrease, or generated offspring).
    #[cfg(not(feature = "parallel"))]
    pub fn insert<E>(&mut self, entity: E)
    where
        // Trait aliases https://github.com/rust-lang/rust/issues/41517
        E: Entity<'e, Kind = K, Context = C> + 'e,
    {
        self.insert_boxed(Box::new(entity));
    }

    /// Inserts the given Entity into the Environment.
    ///
    /// This method is usually used to pre-populate the environment with a set
    /// of entities that will constitute the first generation. After the
    /// environment has been pre-populated the set of entities stored in it will
    /// depend on the behavior of the entities itself (such ad lifespan increase
    /// and decrease, or generated offspring).
    #[cfg(feature = "parallel")]
    pub fn insert<E>(&mut self, entity: E)
    where
        // Trait aliases https://github.com/rust-lang/rust/issues/41517
        E: Entity<'e, Kind = K, Context = C> + 'e + Send + Sync,
    {
        self.insert_boxed(Box::new(entity));
    }

    /// Inserts the given Entity into the Environment.
    fn insert_boxed(&mut self, mut entity: Box<entity::Trait<'e, K, C>>) {
        // insert the weak ref in the grid according to the entity location
        self.tiles.insert(&mut *entity);
        // insert the strong ref in the entities map
        let entities = self.entities.entry(entity.kind()).or_default();
        entities.push(entity);
    }

    /// Draws the environment by iterating over each of its entities, sorted by
    /// kind, and calling the draw method for each one of them.
    ///
    /// Returns an error if any of the draw methods returns an error.
    /// The order of draw calls for each entity of the same type is arbitrary.
    pub fn draw(
        &self,
        ctx: &mut C,
        transform: impl Into<Transform>,
    ) -> Result<(), Error> {
        let transform = transform.into();
        for entities in self.entities.values() {
            for entity in entities {
                entity.draw(ctx, transform)?;
            }
        }
        Ok(())
    }

    /// Returns true only if no Entity is currently in the Environment.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Gets the total number of entities in the environment.
    pub fn count(&self) -> usize {
        self.entities.values().map(|entities| entities.len()).sum()
    }

    /// Gets the total number of entities in the Environment of the given Kind.
    pub fn count_kind(&self, kind: &K) -> usize {
        self.entities
            .get(kind)
            .map(|entities| entities.len())
            .unwrap_or(0)
    }

    /// Gets the current generation step number.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Gets an iterator over all the entities in the Environment.
    ///
    /// The entities will be returned in an arbitrary order.
    pub fn entities(&self) -> impl Iterator<Item = &entity::Trait<'e, K, C>> {
        self.entities
            .values()
            .map(|e| e.iter().map(|e| &**e))
            .flatten()
    }

    /// Gets an iterator over all the (mutable) entities in the Environment.
    ///
    /// The entities will be returned in an arbitrary order.
    pub fn entities_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut entity::Trait<'e, K, C>> {
        self.entities
            .values_mut()
            .map(|e| e.iter_mut().map(|e| &mut **e))
            .flatten()
    }

    /// Gets an iterator over all the entities located at the given location.
    ///
    /// The entities will be returned in an arbitrary order.
    /// The Environment is seen as a Torus from this method, therefore, out of
    /// bounds offsets will be translated considering that the Environment
    /// edges are joined.
    pub fn entities_at(
        &self,
        location: impl Into<Location>,
    ) -> impl Iterator<Item = &entity::Trait<'e, K, C>> {
        self.tiles.entities_at(location)
    }

    /// Gets an iterator over all the (mutable) entities located at the given
    /// location.
    ///
    /// The entities will be returned in an arbitrary order.
    /// The Environment is seen as a Torus from this method, therefore, out of
    /// bounds offsets will be translated considering that the Environment
    /// edges are joined.
    pub fn entities_at_mut(
        &mut self,
        location: impl Into<Location>,
    ) -> impl Iterator<Item = &mut entity::Trait<'e, K, C>> {
        self.tiles.entities_at_mut(location)
    }

    /// Moves forwards to the next generation.
    /// Returns the next generation step number.
    ///
    /// Moving to the next generation involves the following actions:
    /// - Calling `Entity::observe(neighborhood)` for each entity with a snapshot
    ///     of the portion of the environment seen by the entity according to its
    ///     scope. The order of the entities called is arbitrary.
    /// - Calling `Entity::react(neighborhood)` for each entity with a snapshot of
    ///     the portion of the environment seen by the entity according to its
    ///     scope. The order of the entities called is arbitrary.
    /// - Inserting the entities offspring in the environment.
    /// - Removing the entities that reached the end of their lifespan from the
    ///     environment.
    ///
    /// This method will return an error if any of the calls to `Entity::observe()`
    /// or `Entity::react()` returns an error, in which case none of the steps that
    /// involve the update of the environment will take place.
    pub fn nextgen(&mut self) -> Result<u64, Error> {
        self.record_location();
        self.observe_and_react()?;
        self.update_location();

        // take care of newborns entities by inserting them in the environment,
        // as well as removing entities that reached the end of their lifespan
        self.populate_with_offspring();
        self.depopulate_dead();

        self.generation = self.generation.wrapping_add(1);
        Ok(self.generation)
    }

    /// Takes a snapshot of the environment by storing the entities fields that
    /// are going to be updated before moving forward to the next generation.
    fn record_location(&mut self) {
        self.snapshots.clear();
        let additional = self.count().saturating_sub(self.snapshots.capacity());
        self.snapshots.reserve(additional);

        for entities in self.entities.values() {
            for (i, entity) in entities.iter().enumerate() {
                if let Some(location) = entity.location() {
                    self.snapshots.push(Snapshot {
                        id: i,
                        kind: entity.kind(),
                        location,
                    });
                }
            }
        }
    }

    /// Updates the environment according to the current entities and previously
    /// taken snapshot.
    fn update_location(&mut self) {
        // gets the current entity id and location, if the location changed
        let entities = &self.entities;
        let find_entity = |snapshot: &Snapshot<K>| {
            let entity = entities.get(&snapshot.kind)?.get(snapshot.id)?;
            let location = entity.location()?;
            if location != snapshot.location {
                Some((entity.id(), location))
            } else {
                None
            }
        };

        for snapshot in &self.snapshots {
            // update the entity location in the grid of tiles
            if let Some((id, location)) = find_entity(snapshot) {
                debug_assert_ne!(location, snapshot.location);
                self.tiles.relocate(id, snapshot.location, location);
            }
        }
    }

    /// Collects the offspring of all the entities and insert the new entities
    /// in the environment.
    fn populate_with_offspring(&mut self) {
        // gets a list of all the entities offsprings
        let offspring: Vec<Box<entity::Trait<'e, K, C>>> = self
            .entities
            .values_mut()
            .map(|e| e.iter_mut())
            .flatten()
            .filter_map(|e| e.offspring())
            .map(|offspring| offspring.take_entities())
            .flatten()
            .collect();

        // collect entities offsprings and insert them in the environment
        for entity in offspring {
            self.insert_boxed(entity);
        }
    }

    /// Removes all the entities that reached the end of their lifespan.
    fn depopulate_dead(&mut self) {
        for entities in self.entities.values_mut() {
            // remove the weak reference to the entity from the grid of tiles only
            // if it has a location and it reached the end of its lifespan
            for entity in entities.iter() {
                match (entity.location(), entity.lifespan()) {
                    (Some(loc), Some(lifespan)) if !lifespan.is_alive() => {
                        self.tiles.remove(entity.id(), loc);
                    }
                    _ => (),
                };
            }
            // remove the strong reference to the entity if it reached the end
            // of its lifespan
            entities.retain(|entity| {
                if let Some(lifespan) = entity.lifespan() {
                    lifespan.is_alive()
                } else {
                    true
                }
            });
        }
    }

    /// Iterate over each entity and allow them to:
    /// - Execute the provided custom closure the mutable reference of each
    ///     entity.
    /// - Manifest their behavior by calling `Entity::observe(neighborhood)`,
    ///     exposing them to the portion of environment they can see from their
    ///     current location
    /// - For all the same entities, call `Entity::react(neighborhood)`,
    ///     allowing each entity to react to the same portion of the environment.
    /// Returns an error if any of the calls to `Entity::observe()`,
    /// `Entity::react()`, or the provided closure returns an error.
    #[cfg(not(feature = "parallel"))]
    fn observe_and_react(&mut self) -> Result<(), Error> {
        // allow all the entities to observe their neighborhood
        for entities in self.entities.values_mut() {
            for entity in entities.iter_mut() {
                let neighborhood = self.tiles.neighborhood(&**entity);
                entity.observe(neighborhood)?;
            }
        }

        // then allow the same entities to react to the same neighborhoods
        for entities in self.entities.values_mut() {
            for entity in entities.iter_mut() {
                let neighborhood = self.tiles.neighborhood(&**entity);
                entity.react(neighborhood)?;
            }
        }

        Ok(())
    }

    /// Iterate over each entity and allow them to:
    /// - Execute the provided custom closure the mutable reference of each
    ///     entity.
    /// - Manifest their behavior by calling `Entity::observe(neighborhood)`,
    ///     exposing them to the portion of environment they can see from their
    ///     current location
    /// - For all the same entities, call `Entity::react(neighborhood)`,
    ///     allowing each entity to react to the same portion of the environment.
    /// Returns an error if any of the calls to `Entity::observe()`,
    /// `Entity::react()`, or the provided closure returns an error.
    #[cfg(feature = "parallel")]
    fn observe_and_react(&mut self) -> Result<(), Error> {
        use rayon::prelude::*;

        let entities = self
            .entities
            .values_mut()
            .map(|e| e.iter_mut())
            .flatten()
            .map(|e| &mut **e);

        let scheduler::Tasks {
            mut sync,
            mut unsync,
        } = self.scheduler.get_tasks(entities);

        let tiles = &self.tiles;

        // allow all the entities to observe their neighborhood
        sync.par_iter_mut().try_for_each(|entities| {
            for e in entities.iter_mut() {
                let neighborhood = tiles.neighborhood(*e);
                e.observe(neighborhood)?;
            }
            Ok(())
        })?;

        for e in &mut unsync {
            let neighborhood = self.tiles.neighborhood(*e);
            e.observe(neighborhood)?;
        }

        // finally allow the same entities to react to the same neighborhoods
        sync.par_iter_mut().try_for_each(|entities| {
            for e in entities.iter_mut() {
                let neighborhood = tiles.neighborhood(*e);
                e.react(neighborhood)?;
            }
            Ok(())
        })?;

        for e in unsync {
            let neighborhood = self.tiles.neighborhood(e);
            e.react(neighborhood)?;
        }

        Ok(())
    }
}
