# Minesweeper Tutorial

[![pipeline status](https://gitlab.com/qonfucius/incubator/minesweeper/minesweeper-tutorial/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/incubator/minesweeper/minesweeper-tutorial/commits/master)

Source code for the [Bevy tutorial](https://blog.qongzi.com/minesweeper-tutorial) by Félix de Maneville.

<img src="./docs/demo.gif" alt="demo gif" width="400"/>

## Live version

A browser version is available [here](https://qonfucius.gitlab.io/incubator/minesweeper/minesweeper-tutorial/)

## Run

### Native run

use `cargo run` to launch the app in native. Use the `debug` feature for debug inspector and board console output.

### WASM build

* Native: `cargo serve --release` and open `http://127.0.0.1:1334`
* Browser: `./build_wasm.sh` and open `public/index.html` in a browser

## Play

### Board interaction

Use the *left* mouse button to uncover tiles, and the *right* mouse button to mark tiles.

You can also *Clear* the board and *Generate* a new one.