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
   cargo run --release --example mandelbrot
   ```
   <img src="../assets/mandelbrot.gif" width="350" height="200">


 - [Camera](https://en.wikipedia.org/wiki/Transformation_matrix)

    ```bash
    cargo run --release --example camera
    ```
   <img src="../assets/camera.gif" width="300" height="300">
