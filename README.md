## Hypoterra

**A cool game that may never see the light of day.**

`What I upload to this repository is working in some manner.`

## So I remember:

#### Tile System:

The tile system is based on the window size of `1280x960px` which is defined by the constants `WINDOW_WIDTH` and `WINDOW_HEIGHT`. The tilemap is
a 2 dimensional array consisting of `1200` digits (`40x30`). Every digit represents a `32x32px` area in the window. A `1` represents a stone tile and a `0` represents no
tile. As I continue to add different tiles to the game, different tiles will be represented by other numbers.

## Installation:

For the installation I am going to be copying the installation instructions for the SDL2 development tools from the [Tetra Website](http://tetra.seventeencups.net). Tetra is the game engine I am using. After you have completed installing SDL2, move on to the section about installing Hypoterra.

### SDL2 Developer Tools

Installing SDL 2.0
Tetra uses SDL for windowing and input, so you will need to have both the runtime and development libraries installed.

>INFO:
The instructions below are adapted from the README of the sdl2 crate - further information can be found there.

#### Windows
If you're using the default MSVC Rust toolchain:

Go to the [SDL website](https://www.libsdl.org/download-2.0.php) and download the Visual C++ version of the development libraries.

Copy the `.lib` files from the `SDL2-2.0.x/lib/x64` folder of the zip to the `%USERPROFILE/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/x86_64-pc-windows-msvc/lib` folder on your machine. If you are building on a beta/nightly toolchain, adjust the location accordingly.
If you're using the GNU-based Rust toolchain:

Go to the [SDL website](https://www.libsdl.org/download-2.0.php) and download the MinGW version of the development libraries.
Copy the `.lib` files from the `SDL2-2.0.x/x86_64-w64-mingw32/lib` folder of the zip to the `%USERPROFILE/.rustup/toolchains/stable-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib` folder on your machine. If you are building on a beta/nightly toolchain, adjust the location accordingly.
You'll also need to place the SDL2 .dll in the root of your project (and alongside your .exe when distributing your game). You can download this from the 'Runtime Binaries' section of the SDL website.

#### Mac
The easiest way to install SDL is via [Homebrew](https://brew.sh/):

`brew install sdl2`

You will also need to add the following to your `~/.bash_profile`, if it is not already present.

`export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"`

#### Linux

The SDL development libraries are distributed through most Linux package managers - here are a few examples:

##### Ubuntu/Debian

`sudo apt install libsdl2-dev`

##### Fedora/CentOS

`sudo yum install SDL2-devel`

##### Arch Linux

`sudo pacman -S sdl2`

##### Installing ALSA (Linux only)

On Linux, ALSA is used as the audio backend, so you will also need the ALSA development libraries installed. Similar to SDL, you can find these libraries on most Linux package managers:

##### Ubuntu/Debian

`sudo apt install libasound2-dev`

##### Fedora/CentOS

`sudo yum install alsa-lib-devel`

##### Arch Linux

`sudo pacman -S alsa-lib`

### Installing Hypoterra

First, clone the repository:

`git clone https://github.com/gegs921/hypoterra.git`

Next, change the directory to hypoterra:

`cd hypoterra`

Then, with all of the proper tools installed, run:

`cargo build --release`

Then to run the game:

**Mac/Linux:**

`./target/release/hypoterra`

**Windows:**

`./target/release/hypoterra.exe`
