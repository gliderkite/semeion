//! An experimental 2D environment simulator, that let's you define the behavior
//! and the shape of your entities, while taking care of dispatching events
//! generation after generation.
//!
//! # Motivation
//! `semeion` is a library that was born out of the curiosity to see
//! how to abstract those few concepts that are, most of the times, shared
//! between very simple 2D games mostly focused on simulations, such as cellular
//! automata or zero-player games.
//! When writing such "home-made" games, it's usually standard practice to rely
//! on already existing game engines, which do a good job in abstracting the
//! complexity of the event loop, graphic rendering system, or assets management;
//! they all come in different flavors, but they mostly share the same concepts
//! when it comes to event handling: the *update* callback allows you to define
//! where the logic of your game will take place, and the *draw* callback allows
//! you to define where the rendering of your entities is going to happen;
//! finally the third main component regards the player's input events.
//!
//! This is great for the developer, the simplicity and feature richness of game
//! engines such as [SFML](https://www.sfml-dev.org/) or [ggez](https://ggez.rs/),
//! just to name a couple, allowed many developers to write their own Atari Pong
//! version.
//!
//! But besides the event handling, the graphics rendering system, and the assets
//! management, I noticed that writing small games, especially when focusing on
//! simulations and similar, always involved another type of abstraction that
//! was shared and re-implemented several times in each of these game variants:
//! the entities management.
//!
//! While there are already several systems capable of providing you with an
//! abstraction to take care of your entities, and many of these are
//! proven to be very efficient and scale well in the industry by exploiting
//! patterns like the Entity-Component-System (ECS) that you can find for example
//! in [Amethyst](https://amethyst.rs/), I wasn't satisfied with the current
//! state of the art like I was with game engines focused on very
//! small projects (and obviously :) I wanted to write my own).
//!
//! This is where `semeion` takes place; it's a very simple and very immature
//! framework that acts orthogonally to your game engine, and allows you to
//! focus on the behavior of your entities while it takes care of dispatching the
//! events that are strictly related to the entities themself during their own
//! lifetime, which belong to the 2D environment shaped like a grid of squared
//! tiles of the same size, generation after generation.
//!
//! With `semeion`, you can implement the generic [Entity](entity/trait.Entity.html)
//! trait and define the behavior of your entities for each kind, and how they
//! will interact with each other according to their scope of influence,
//! location in the [Environment](env/struct.Environment.html), and lifetime.
//!
//! Even though `semeion` has been developed with `ggez` as a companion game
//! engine, it is independent from it, and can be easily used with the game
//! engine of your preference.
//! Check out the *examples* folder to see how to use the library.

#![allow(clippy::type_complexity)]

mod math;

pub use entity::*;
pub use env::*;
pub use lifespan::*;
pub use space::*;

pub mod entity;
pub mod env;
pub mod lifespan;
pub mod space;
