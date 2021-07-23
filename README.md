# Paddle - Easy 2D browser games in Rust

<img alt="Image: Paddle logo" src="./examples/loading_images/www/paddle_icon.svg" width="40%">

Paddle is a game engine to make simple games for the web without headache.

I started this project in 2019 because I was baffled how hard it was to get the simplest Rust game running in the browser. I struggled especially with things like interactive UI elements. This is very easy from JavaScript, so why is it so hard from Rust?

So I started to experiment how I can bring the DOM and my Rust code closer together. The initial experimentation happens in projects outside this crate. Only once I see that a certain approach works, I will port it to Paddle and try to fit it in with the existing features. Hopefully, this will lead to a set of practically useful features. The final goals it to make building browser games with Rust as easy as it is with JavaScript!

There is still a long way for Paddle. At the same time, the browser support in other Rust game engines is getting better with every release and the community is very active. I am more than happy to see that! But so far, I still feel like Paddle would fill a niche of a purely web-focused engine which does not really exist, yet. So I keep at it for the forseeable future.

The exact scope Paddle will entail is not set in stone, yet. However, I try to keep this document updated with current plans for this crate.

**0.1.0 Beta now published but API highly unstable. This is only released to crates.io for [Paddlers](https://github.com/jakmeier/paddlers-browser-game), which uses this in the live-demo.**

## Design Principles
In the landscape of game engine design choices, Paddle makes the following tradeoffs:
* Focus on the best experience in web browsers and ignore other platforms entirely.
* Simplicity trumps performance and cleverness.
* Make the API as simple as possible to use **for newbies**. With that I mean anyone new to game development, new to Rust, or even new to programming in general. This is often different from *simple for experienced programmers*.

## Goals Statement
A bit more specifically, the principles translate into the following goals for the crate.
* Highest priority features in the engine are those that make the developers' life easier. For example, provide a solution for loading assets over the network, display and style texts, create clickable images, ...
* Harness native browser capabilities like HTML + CSS user interfaces, SVG rendering, and more to come in the future.
* Compatibility with as many browser versions and devices as possible (Only WebGL 1 + WASM required, touch and mouse support for all browsers, etc.)
* The library should become beginner-friendly enough that it can be recommended for people's first Rust project to learn the language and for game jams where not everybody in the team knows a lot of Rust.
* Avoid complex public function signatures, such as traits with too many generic parameters.
* Simple programming interfaces over maximum efficiency when there is a conflict.
* Write easy-to-follow documentation.
* 2D graphics only (3D could be added later if it does not pollute the 2D API.)

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