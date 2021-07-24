use paddle::quicksilver_compat::*;
use paddle::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn start() {
    // Build configuration object to define all setting
    let config = PaddleConfig::default()
        .with_canvas_id("paddle-canvas-id")
        .with_background_color(GREY)
        .with_resolution((960, 540));

    // Start game engine
    paddle::init(config).expect("Paddle initialization failed.");

    // Create our game state and register it
    let state = ();
    paddle::register_frame(Game {}, state, (0, 0));
}

struct Game {}

impl paddle::Frame for Game {
    // This simple example does not need any shared state variables.
    type State = ();
    // This defines the size of the frame (in game coordinates)
    const WIDTH: u32 = 960;
    const HEIGHT: u32 = 540;

    // Will get called ~60 times per second, or might be adapted to the screen refresh rate. (Browser will decide)
    fn draw(&mut self, _state: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        // Adapt canvas size to viewport on every frame
        canvas.fit_display(10.0);

        // Spinning white square, 50 degree/s
        let rect = Rectangle::new((-70, -70), (140, 140));
        let transform =
            Transform::translate((480, 270)) * Transform::rotate((timestamp / 20.0) as f32 % 360.0);
        let z = 1;
        canvas.draw_ex(&rect, &Color::WHITE, transform, z);
    }
}

const GREY: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
