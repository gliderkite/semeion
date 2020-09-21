# semeion

A 2D environment simulator, that let's you define the behavior and the shape
of your entities, while taking care of dispatching events generation after
generation.

[![docs.rs](https://docs.rs/semeion/badge.svg)](https://docs.rs/semeion)
[![crates.io](https://img.shields.io/crates/v/semeion.svg)](https://crates.io/crates/semeion)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)


## Overview

For an overview of what you can do, and how you can do it with this library, the
best way to start is to have a look at the several [examples](#examples) that come
with this project.
Each of these aims to show a subset of the features set of `semeion`, while
giving an example of how to make the best use of it.
While, if you want to have a look at a more complete project that uses
`semeion` as its entity engine, check out
[formicarium](https://github.com/gliderkite/formicarium).

Note: if you want to clone this repository without having to download the
.gif assets you can do so with:
```bash
git clone --single-branch https://github.com/gliderkite/semeion.git
```

## Optional Features

By default, `semeion` uses an environment engine that schedules all the entity
callbacks on the same single thread, therefore no synchronization is required
(no user's implemented `Entity` needs to be sent or shared between threads).

Nevertheless, there may be scenarios in which you are running a simulation that
includes a significant number of entities *and* each of these entity tasks,
required to proceed to the next generation, is considerably resource consuming.
For these situations, it is possible to gain significant advantage by spawning
multiple threads and running the simulation in parallel (profiling your code is
always advised before taking final decisions).

At the moment, you can enable this feature only at compile time, by specifying
the optional feature `parallel` in your `Cargo.toml`:

```toml
semeion = { version = "0.8", features = ["parallel"] }
```

The only requirement is that all your entities need to be
[Send](https://doc.rust-lang.org/std/marker/trait.Send.html) and
[Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html).


## Examples

 - [Langton's Ant](https://en.wikipedia.org/wiki/Langton%27s_ant)

   ```bash
   cargo run --release --example langton
   ```
   <img src="../assets/langton.gif" width="300" height="240">

 - [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

   ```bash
   cargo run --release --example life
   ```
   <img src="../assets/life.gif" width="350" height="300">


 - [Wireworld](https://en.wikipedia.org/wiki/Wireworld)

   ```bash
   cargo run --release --example wireworld
   ```
   <img src="../assets/wireworld.gif" width="300" height="300">


 - [Mandelbrot](https://en.wikipedia.org/wiki/Mandelbrot_set)

   ```bash
   cargo run --release --example mandelbrot --features parallel
   ```
   <img src="../assets/mandelbrot.gif" width="350" height="200">


 - [Camera](https://en.wikipedia.org/wiki/Transformation_matrix)

    ```bash
    cargo run --release --example camera
    ```
   <img src="../assets/camera.gif" width="300" height="300">
