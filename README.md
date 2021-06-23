# Minesweeper Tutorial

[![pipeline status](https://gitlab.com/qonfucius/incubator/minesweeper/minesweeper-tutorial/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/incubator/minesweeper/minesweeper-tutorial/commits/master)

Source code for the Bevy tutorial by Félix de Maneville.

<img src="./docs/demo_dark.gif" alt="demo gif" width="400"/>

## Run

use `cargo run` to launch the app in native. Use the `debug` feature for debug inspector and board console output.

### Using CMake

* Native: `cargo make run`
* Browser (wasm): `cargo make serve` 

> a `release` profile is configured

## Play

### Board interaction

Use the *left* mouse button to uncover tiles, and the *right* mouse button to mark tiles.

You can also *Clear* the board, *Generate* a new one or switch themes

<img src="./docs/light_theme_screen.png" alt="screenshot" width="300"/>
<img src="./docs/dark_theme_screen.png" alt="dark screenshot" width="300"/>

## Board edition

The application loads its theme from files in the `assets` folder (see `main.rs`).
You may edit the files or edit the `main.rs` file