# Java translated to Rust

Translated 0 of 0 methods and 0 of 0 static fields to Rust. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Every class


### `ModrinthWrapper`

