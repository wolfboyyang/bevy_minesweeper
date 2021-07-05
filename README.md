# Minesweeper Tutorial

[![pipeline status](https://gitlab.com/qonfucius/incubator/minesweeper/minesweeper-tutorial/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/incubator/minesweeper/minesweeper-tutorial/commits/master)

Source code for the Bevy tutorial by Félix de Maneville.

<img src="./docs/demo.gif" alt="demo gif" width="400"/>

## Live version

A browser version is available [here](https://qonfucius.gitlab.io/incubator/minesweeper/minesweeper-tutorial/)

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

<img src="./docs/light_theme_screen.png" alt="screenshot" width="400"/>
<img src="./docs/dark_theme_screen.png" alt="dark screenshot" width="400"/>