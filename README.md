# Xtarda Rescue

![Screen shot](https://www.martyndavis.com/wp-content/uploads/2022/12/xtarda.png "")

## Rationale
Just a bit more retro fun as a Rust learning exercise. This is a game I wrote originally
for the ZX Spectrum about forty years ago.

## Playing the game
The object is to rescue people from the hostile planet Xtarda. You are orbiting in your
mothership and you can drop pods with the down arrow key. You can steer the pod left
and right as it drops with the left and right arrow keys. You must avoid colliding with any
of the asteroids below. Once the pod has landed (on the
landing pad only), you can then take off again with the up arrow key to attempt to return
to the mothership with one rescued person each time.

## To-Do List
* Maybe animation of rescuee floating back down if pod crashes while ascending

## Pre-requisites
### Linux

Ensure you have `libcsfml-dev` and `libsfml-dev` installed.

This runs surprisingly well in WSLg, but very, very slowly in regular WSL2 (plus regular
WSL2 won't support the sounds and will cause the program to crash).

### Mac

`brew install sfml` and `brew install csfml` (install homebrew if you haven't already)

then `export SFML_INCLUDE_DIR=/System/Volumes/Data/opt/homebrew/include` and `export SFML_LIBS_DIR=/System/Volumes/Data/opt/homebrew/lib`


## Building
Just the usual `cargo build`.

## Running
Just the usual `cargo run`.

## Running unit tests
Just the usual `cargo test`.

