# Paddle - Easy 2D browser games in Rust

<img alt="Image: Paddle logo" src="./examples/loading_images/www/paddle_icon.svg" width="40%">

**0.1.0 Beta now published but API highly unstable. This is only released to crates.io for [Paddlers](https://github.com/jakmeier/paddlers-browser-game), which uses this in the live-demo.**

## Goals Statement
 * For the web only
 * Allow using browser capabilities like HTML + CSS user interfaces, native SVG rendering, and more to come in the future
 * Compatibility with as many browser versions and devices as possible (Only WebGL 1 + WASM required, touch and mouse support for all browsers, etc.)
 * 2D graphics only for now, 3D unlikely to be included any time soon
 * Simple programming interfaces over maximum efficiency when there is a conflict. 
 * The library should eventually be beginner-friendly enough that it can be recommended for people learning Rust, gamedev, or even programming in general.

## Technical Overview

### Implemented in Paddle
* Cross-browser input
* Screen resizing
* Custom WebGL layer
* Support for basic geometries and image drawing (Originally taken from [quicksilver](https://github.com/ryanisaacg/quicksilver) and adapted)
* Support for text placement on the screen
* Activity (frame) management

### External Dependencies
* [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) / [web-sys](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys) / [wasm-pack](https://github.com/rustwasm/wasm-pack) for deployment on the web
* [div-rs](https://github.com/jakmeier/div-rs) for integration of HTML components
* [nuts](https://github.com/jakmeier/nuts) for activity management and (seemingly stateless) point-to-point messages and broadcasts

### Core Principles
**Frames**
Everything lives in so-called *frames*. These are activities that also occupy a space on the screen, where they can draw and receive user input.
Multiple frames can be used to create UIs or to switch between scenes easily.

*Work in progress, documentation will be added as the project matures...*