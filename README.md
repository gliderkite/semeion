# semeion

An experimental 2D environment simulator, that let's you define the behavior and
the shape of your entities, while taking care of dispatching events generation
after generation.

[![docs.rs](https://docs.rs/semeion/badge.svg)](https://docs.rs/semeion)
[![crates.io](https://img.shields.io/crates/v/semeion.svg)](https://crates.io/crates/semeion)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)


## Overview

For an overview of what you can do, and how you can do it with this library, the
best way to start is to have a look at the many [examples](#examples) that come
with this project.
Each of these aims to show a subset of the features set of `semeion`, while
giving an example of how to make the best use of it.


## Examples

 - [Langton's Ant](https://en.wikipedia.org/wiki/Langton%27s_ant)

    ```bash
    cargo run --release --example langton
    ```

    ![langton preview](../assets/langton.gif)

 - [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

    ```bash
    cargo run --release --example life
    ```

    ![life preview](../assets/life.gif)


 - [Wireworld](https://en.wikipedia.org/wiki/Wireworld)

    ```bash
    cargo run --release --example wireworld
    ```

    ![wireworld preview](../assets/wireworld.gif)


 - [Mandelbrot](https://en.wikipedia.org/wiki/Mandelbrot_set)

    ```bash
    cargo run --release --example mandelbrot
    ```

    ![mandelbrot preview](../assets/mandelbrot.gif)



## Semantic Versioning

This library is extremely experimental, but it follows the basic rules of
[semantic versioning](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field).