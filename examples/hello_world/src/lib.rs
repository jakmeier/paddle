use paddle::quicksilver_compat::*;
use paddle::{PaddleConfig, WebGLCanvas};
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
pub fn start() {
    // Build configuration object to define all setting
    let config = PaddleConfig::default()
        .with_canvas_id("paddle-canvas-id")
        .with_resolution((960, 540));

    // Start game engine
    paddle::init(config).expect("Paddle initialization failed.");

    // Create our game state and register it
    let state = ();
    paddle::register_frame(Game {}, state);
}

struct Game {}

impl paddle::Frame for Game {
    type Error = ();
    type State = ();

    // Will get called ~60 times per second, or might be adapted to the screen refresh rate. (Browser will decide)
    fn draw(
        &mut self,
        _state: &mut Self::State,
        canvas: &mut WebGLCanvas,
        timestamp: f64,
    ) -> Result<(), Self::Error> {
        // Adapt canvas size to viewport on every frame
        canvas
            .fit_to_screen(10.0)
            .expect("Failed fitting to screen");

        // Grey background
        let grey = Color {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        };
        canvas.clear(grey);

        // Spinning white square, 50 degree/s
        let rect = Rectangle::new((410, 200), (140, 140));
        let transform = Transform::rotate((timestamp / 20.0) as f32 % 360.0);
        let z = 1.0;
        canvas.draw_ex(&rect, Col(Color::WHITE), transform, z);

        Ok(())
    }
}
