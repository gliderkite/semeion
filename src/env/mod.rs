use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use super::*;
use tile::*;

mod neighborhood;
mod tile;

pub use neighborhood::*;

/// Unordered map of entities identified by their IDs, where all the entities
/// belongs to the same Kind.
type Entities<I, K, C, T, E> =
    HashMap<I, entity::EntityStrongRef<I, K, C, T, E>>;

/// Sorted map of all the entities by Kind.
type EntitiesKinds<I, K, C, T, E> = BTreeMap<K, Entities<I, K, C, T, E>>;

/// The Environment is a grid, of squared tiles with the same size, where all
/// the entities belongs. The Environment acts both as the data structure as
/// well as the engine that controls the behavior of all the entities in it,
/// and their interaction for each generation. Where both of those are defined
/// by the user, and handled via the Entity trait method called for each entity
/// by the environment generation after generation.
/// An environment can contains entities of different kinds, dynamically
/// allocated and defined according to the user needs via the Entity trait, but
/// all the entities must share the same Entity trait associated types.
/// The Environment can be created with specific bounds, that represents the
/// size of the grid that describes its geometry (the Bounds can be computed
/// from the window size in pixels where the environment will be drawn for a
/// specific length of the grid tiles side).
/// Once the environment is initialized by inserting entities in its initial
/// population, it can be drawn by drawing all its entities, and it is possible
/// to move to the next generation (where the interaction between the entities
/// takes place).
/// The geometry of the environment is defined as a Torus, that is, the grid
/// bounds are adjacent to each other, allowing therefore the entities to move
/// past each bounds into the next tile as if there were no limits.
#[derive(Debug)]
pub struct Environment<
    I: Eq + Hash + Clone + Debug,
    K: Eq + Hash + Debug,
    C,
    T,
    E,
> {
    // the list of strong references to the entities
    entities: EntitiesKinds<I, K, C, T, E>,
    // the (1-dimensional) grid of tiles that stores week references to the
    // entities according to their location
    tiles: Tiles<I, K, C, T, E>,
    // the environment bounds
    bounds: Bounds,
    // the latest snapshot of the environment, used to update the entities
    // properties within it at each generation
    snapshots: Vec<Snapshot<I, K>>,
    // the generation counter
    generation: u64,
}

#[derive(Debug)]
struct Snapshot<I, K> {
    id: I,
    kind: K,
    location: Location,
}

impl<I: Eq + Hash + Clone + Debug, K: Eq + Hash + Ord + Debug, C, T, E>
    Environment<I, K, C, T, E>
{
    /// Constructs a new environment with the given bounds. The bounds represent
    /// the size of the grids of squared tiles of same side length, as number of
    /// columns and rows.
    pub fn new(bounds: Bounds) -> Self {
        Self {
            entities: BTreeMap::new(),
            tiles: Tiles::new(bounds),
            bounds,
            snapshots: Vec::new(),
            generation: 0,
        }
    }

    /// Inserts the given Entity into the Environment.
    /// This method is usually used to pre-populate the environment with a set
    /// of entities that will constitute the first generation. After the
    /// environment has been pre-populated the set of entities stored in it will
    /// depend on the behavior of the entities itself (such ad lifespan increase
    /// and decrease, or generated offspring).
    pub fn insert<Q>(&mut self, entity: Q)
    where
        Q: Entity<Id = I, Kind = K, Context = C, Transform = T, Error = E>,
        Q: 'static,
    {
        self.insert_entity(Rc::new(RefCell::new(entity)));
    }

    /// Draws the environment by iterating over each of its entities, sorted by
    /// kind, and calling the draw method for each one of them. Returns an error
    /// if any of the draw methods returns an error. The order of draw calls for
    /// each entity of the same type is arbitrary.
    pub fn draw(&self, ctx: &mut C, transform: &T) -> Result<(), E> {
        for entities in self.entities.values() {
            for entity in entities.values() {
                entity.borrow().draw(ctx, transform)?;
            }
        }
        Ok(())
    }

    /// Gets the total number of entities in the environment.
    pub fn count(&self) -> usize {
        self.entities.values().map(|entities| entities.len()).sum()
    }

    /// Gets the total number of entities in the environment by kind.
    pub fn count_by_kind(&self) -> HashMap<&K, usize> {
        self.entities
            .iter()
            .map(|(kind, entities)| (kind, entities.len()))
            .collect()
    }

    /// Gets the current generation step number.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Moves forwards to the next generation.
    /// Moving to the next generation involves the following actions sorted by
    /// order:
    /// - Calling `Entity::act(neighborhood)` for each entity with a snapshot of
    ///     the portion of the environment seen by the entity according to its
    ///     scope. The order of the entities called is arbitrary.
    /// - Inserting the entities offspring in the environment.
    /// - Removing the entities that reached the end of their lifespan from the
    ///     environment.
    ///
    /// This method will return an error if any of the calls to `Entity::act()`
    /// returns an error, in which case none of the steps that involve the update
    /// of the environment will take place.
    /// Nevertheless, it is guaranteed that all the calls to `Entity::act()` will
    /// be performed independently of their outcome, in which case the first
    /// error generated will be returned (even in case of multiple errors).
    pub fn nextgen(&mut self) -> Result<(), E> {
        // call Entity::act(neighborhood) for each entity and update the environment
        // accordingly only after all the entities have been iterated, by relying
        // on a previously taken snapshot of the environment
        self.take_snapshot();
        self.act()?;
        self.update();

        // take care of newborns entities by inserting them in the environment,
        // as well as removing entities that reached the end of their lifespan
        self.populate_with_offspring();
        self.depopulate_dead();

        self.generation += 1;
        Ok(())
    }

    /// Inserts a new entity in the environment according to its location.
    fn insert_entity(&mut self, entity: EntityStrongRef<I, K, C, T, E>) {
        let (id, kind) = {
            let entity = entity.borrow();
            (entity.id().clone(), entity.kind())
        };
        // insert the weak ref in the grid according to the entity location
        self.tiles.insert(&entity);
        // insert the strong ref in the entities map
        let entities = self.entities.entry(kind).or_default();
        entities.insert(id, entity);
    }

    /// Takes a snapshot of the environment by storing the entities fields that
    /// are going to be updated before moving forward to the next generation.
    fn take_snapshot(&mut self) {
        self.snapshots.clear();
        let additional = self.count().saturating_sub(self.snapshots.capacity());
        self.snapshots.reserve(additional);

        for entities in self.entities.values() {
            for entity in entities.values() {
                let entity = entity.borrow();
                // if the entity has no location there is nothing to update in
                // the environment
                if let Some(location) = entity.location() {
                    self.snapshots.push(Snapshot {
                        id: entity.id().clone(),
                        kind: entity.kind(),
                        location,
                    });
                }
            }
        }
    }

    /// Updates the environment according to the current entities and previously
    /// taken snapshot.
    fn update(&mut self) {
        for snapshot in &self.snapshots {
            // gets the current entity location
            let get_location = || {
                self.entities
                    .get(&snapshot.kind)?
                    .get(&snapshot.id)?
                    .borrow()
                    .location()
            };
            // update the entity location in the grid of tiles
            if let Some(location) = get_location() {
                self.tiles.swap(&snapshot.id, snapshot.location, location);
            }
        }
    }

    /// Collects the offspring of all the entities and insert the new entities
    /// in the environment.
    fn populate_with_offspring(&mut self) {
        // gets a list of all the entities offsprings
        let offspring: Vec<EntityStrongRef<I, K, C, T, E>> = self
            .entities
            .values()
            .map(|e| e.values())
            .flatten()
            .filter_map(|e| e.borrow_mut().offspring())
            .map(|offspring| offspring.take_entities())
            .flatten()
            .collect();

        // collect entities offsprings and insert them in the environment
        for entity in offspring {
            self.insert_entity(entity);
        }
    }

    /// Removes all the entities that reached the end of their lifespan.
    fn depopulate_dead(&mut self) {
        for entities in self.entities.values_mut() {
            // remove the weak reference to the entity from the grid of tiles only
            // if it has a location and it reached the end of its lifespan
            for entity in entities.values() {
                let e = entity.borrow();
                match (e.location(), e.lifespan()) {
                    (Some(loc), Some(lifespan)) if !lifespan.is_alive() => {
                        self.tiles.remove(e.id(), loc);
                    }
                    _ => (),
                };
            }
            // remove the strong reference to the entity if it reached the end
            // of its lifespan
            entities.retain(|_, entity| {
                if let Some(lifespan) = entity.borrow().lifespan() {
                    lifespan.is_alive()
                } else {
                    true
                }
            });
        }
    }

    /// Iterate over each entity and allow them to manifest their behavior by
    /// calling Entity::act(neighborhood) exposing them to the portion of
    /// environment they can see from their current location. Returns an error
    /// if any of the calls to `Entity::act()` returns an error, nevertheless
    /// it is guaranteed that all the calls to `Entity::act()` will be performed
    /// for the current generation, in which case the first generated error is
    /// returned.
    fn act(&mut self) -> Result<(), E> {
        let mut res = Ok(());
        for entities in self.entities.values_mut() {
            for entity in entities.values_mut() {
                let mut e = entity.borrow_mut();
                let neighborhood = self.tiles.neighborhood(&e);
                res = res.and(e.act(neighborhood));
            }
        }
        res
    }
}
