# Simple CLI game engine in Rust

### Description

Main abstraction of the engine is trait `Object`.
Each object can be drawn on screen and handles some interactions.

Currently added types that implement `Object` are:
##### Items
* `Food`
* `Weapon`
##### Entities
* `Enemy`
* `Player`
##### Other
* `Door`

Main container for game objects is struct `Level`.
Player selects the target from level's content and issues an action, that `Level::handle()` handles.

Struct `Game` holds reference to the player. It is responsible for running the loop and parsing input.

### Prerequisities

To build the project, aswell as docs, you need rust compiler `rustc` and `cargo`. They are both packaged in rust's build system `rustup`.

Installation guide: https://doc.rust-lang.org/cargo/getting-started/installation.html

Alternatively, install it using your distribution's package manager.

For Arch Linux:
```
[refresh the database and install rustup]
# pacman -Sy rustup

[to install default toolchain and setup compiler]
$ rustup default stable
```


In order to build and run the engine, simply type `cargo run` in the root diectory of the project.
### Documentation

For more information, check out the docs.
They can be viewed directly from the code, or built using cargo.

To open docs in your browser use:
```
$ cargo doc --no-deps --open
```
Or if you want to see private properties aswell:
```
$ cargo doc --no-deps --open --document-private-items
```

### Author information
This project was made by Dominik Muc, student at University of Wrocław.

My index is 345952. Email: 345952 [at] uwr.edu.pl

For copyright notice, check LICENSE at the root of project's directory.
