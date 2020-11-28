//! A 2D environment simulator, that let's you define the behavior and the shape
//! of your entities, while taking care of dispatching events generation after
//! generation.
//!
//! # Overview
//! `semeion` is a library that was born out of the curiosity to see
//! how to abstract those few concepts that are, most of the times, shared
//! between very simple 2D games mostly focused on simulations, such as cellular
//! automata or zero-player games.
//!
//! When writing such games, it's usually standard practice to rely on already
//! existing game engines, which do a great job in abstracting the complexity of
//! the event loop, graphic rendering system, or assets management.
//! They all come in different flavors, but they mostly share the same concepts
//! when it comes to event handling: the *update* callback allows you to define
//! where the logic of your game will take place, and the *draw* callback allows
//! you to define where to render your entities; finally the third main component
//! takes care of the player's inputs.
//!
//! But besides the events handling, the graphics rendering system, or the assets
//! management, writing small games, especially when focusing on simulations
//! and similar, most often involves another type of abstraction: the entities
//! management system and related components.
//!
//! This is where `semeion` takes place: it's a basic framework that acts
//! orthogonally and independently from your game engine, and allows you to focus
//! on the behavior of your entities, while it takes care of managing them and
//! dispatching their events.
//!
//! With `semeion`, you can implement the generic [Entity](crate::Entity)
//! trait and define the behavior of your entities for each kind, and how they
//! will interact with each other according to their scope of influence,
//! location in the [Environment](crate::Environment), and lifetime.

pub use entity::*;
pub use env::*;
pub use error::*;
pub use math::*;
pub use space::*;

pub mod entity;
pub mod env;
pub mod error;
pub mod math;
pub mod space;
