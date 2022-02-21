# Minesweeper Tutorial

[![pipeline status](https://gitlab.com/qonfucius/minesweeper-tutorial/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/minesweeper-tutorial/commits/master)

Source code for the [Bevy Minesweeper Tutorial](https://dev.to/qongzi/bevy-minesweeper-introduction-4l7f) by Félix de Maneville:
- [Introduction](https://dev.to/qongzi/bevy-minesweeper-introduction-4l7f)
- [Project Setup](https://dev.to/qongzi/bevy-minesweeper-part-1-534c)
- [Tile Map Generation](https://dev.to/qongzi/bevy-minesweeper-part-2-1hi5)
- [Spawning the board](https://dev.to/qongzi/bevy-minesweeper-part-3-1a9a)
- [Tiles and Components](https://dev.to/qongzi/bevy-minesweeper-part-4-2co9)
- [Interaction and the board resource](https://dev.to/qongzi/bevy-minesweeper-part-5-24j4)
- [Covering and Uncovering tiles](https://dev.to/qongzi/bevy-minesweeper-part-6-46jh)
- [Placing a safe start](https://dev.to/qongzi/bevy-minesweeper-part-7-1ko2)
- [Generic states](https://dev.to/qongzi/bevy-minesweeper-part-8-4apn)
- [Board dynamic themes](https://dev.to/qongzi/bevy-minesweeper-part-9-534e)
- [Gameplay completion](https://dev.to/qongzi/bevy-minesweeper-part-10-5hie)
- [WASM support](https://dev.to/qongzi/bevy-minesweeper-part-11-3aim)

<img src="./docs/demo.gif" alt="demo gif" width="400"/>

## Live version

A browser version is available [here](https://qonfucius.gitlab.io/minesweeper-tutorial/)

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