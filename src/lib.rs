//! A 2D environment simulator, that let's you define the behavior and the shape
//! of your entities, while taking care of dispatching events generation after
//! generation.
//!
//! # Overview
//! `semeion` is a library that was born out of the curiosity to see
//! how to abstract those few concepts that are, most of the times, shared
//! between very simple 2D games mostly focused on simulations, such as cellular
//! automata or zero-player games.
//! When writing such games, it's usually standard practice to rely on already
//! existing game engines, which do a great job in abstracting the complexity of
//! the event loop, graphic rendering system, or assets management;
//! they all come in different flavors, but they mostly share the same concepts
//! when it comes to event handling: the *update* callback allows you to define
//! where the logic of your game will take place, and the *draw* callback allows
//! you to define where the rendering of your entities is going to happen;
//! finally the third main component regards the player's input events.
//!
//! This is great for the developer, the simplicity and feature richness of game
//! engines such as [SFML](https://www.sfml-dev.org/) or [ggez](https://ggez.rs/),
//! just to name a couple, allows many developers to write their own Atari Pong
//! version and much more.
//!
//! But besides the event handling, the graphics rendering system, and the assets
//! management, writing small games, especially when focusing on simulations
//! and similar, most often involves another type of abstraction that
//! is shared and re-implemented several times in each of these games variants:
//! the entities management system and related components.
//!
//! This is where `semeion` takes place; it's a very basic framework that acts
//! orthogonally to your game engine, and allows you to focus on the
//! behavior of your entities, while it takes care of dispatching the entities
//! related events during their own lifetime.
//!
//! With `semeion`, you can implement the generic [Entity](entity/trait.Entity.html)
//! trait and define the behavior of your entities for each kind, and how they
//! will interact with each other according to their scope of influence,
//! location in the [Environment](env/struct.Environment.html), and lifetime.

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
