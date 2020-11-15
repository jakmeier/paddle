use paddle::graphics::{Image, TextureConfig};
use paddle::quicksilver_compat::*;
use paddle::{LoadScheduler, PaddleConfig, WebGLCanvas};
use wasm_bindgen::prelude::wasm_bindgen;

const SCREEN_W: f32 = 1920.0;
const SCREEN_H: f32 = 1080.0;

// #e57c40ff
const PADDLE_ORANGE: Color = Color {
    r: 0xe5 as f32 / 255.0,
    g: 0x7c as f32 / 255.0,
    b: 0x40 as f32 / 255.0,
    a: 1.0,
};

#[wasm_bindgen]
pub fn start() {
    // Build configuration object to define all setting
    let texture_config = TextureConfig::default().without_filter().with_rgba();
    let config = PaddleConfig::default()
        .with_canvas_id("paddle-canvas-id")
        .with_resolution((SCREEN_W, SCREEN_H))
        .with_texture_config(texture_config);

    // Start game engine
    paddle::init(config).expect("Paddle initialization failed.");

    // Define images to load
    let icon = Image::load("paddle_icon.svg");
    let background = Image::load("background.png");
    let paddlers_icon = Image::load("paddlers.svg");

    // Image loadings is asynchronous, hence we only have Future<Result<Image>> objects so far.
    // We could await the futures, doing nothing until the images arrive.
    // Or we can make use of the LoadScheduler to track loading progress and continue once all has been loaded.
    // TODO:
    // let images = vec![icon, background];
    // let load_manager = LoadScheduler::new().with_vec(images, "Loading images...");

    // Quick version for now, awaiting futures one by one:
    let future = async move {
        let state = GlobalState {
            icon: icon.await.expect("loading icon failed"),
            background: background.await.expect("loading background failed"),
            paddlers_icon: paddlers_icon.await.expect("loading background failed"),
        };
        // Create our game state and register it
        paddle::register_frame(Game {}, state);
    };

    wasm_bindgen_futures::spawn_local(future);
}

struct Game {}
struct GlobalState {
    icon: Image,
    background: Image,
    paddlers_icon: Image,
}

impl paddle::Frame for Game {
    type Error = paddle::ErrorMessage;
    type State = GlobalState;

    // Will get called ~60 times per second, or might be adapted to the screen refresh rate. (Browser will decide)
    fn draw(
        &mut self,
        global: &mut Self::State,
        canvas: &mut WebGLCanvas,
        timestamp: f64,
    ) -> Result<(), Self::Error> {
        canvas.fit_to_screen(10.0)?;
        canvas.clear(PADDLE_ORANGE);

        // Background image filling the screen
        canvas.draw(
            &Rectangle::new_sized((SCREEN_W, SCREEN_H)),
            &global.background,
        );

        // Large icon in the center
        let icon_s = 500.0;
        let center = Vector::new(SCREEN_W / 2.0, SCREEN_H / 2.0);
        draw_at_center(canvas, &global.icon, center, icon_s);

        // Small icons orbiting the large one
        let small_icon_s = icon_s / 1.618f32.powi(2);
        let r = 450.0;

        let period = timestamp / 3000.0;
        let direction = Vector::new(period.cos(), period.sin());
        let pos = center + direction * r;
        draw_at_center(canvas, &global.icon, pos, small_icon_s);

        let period = timestamp / 3000.0 + std::f64::consts::PI;
        let direction = Vector::new(period.cos(), period.sin());
        let pos = center + direction * r;
        draw_at_center(canvas, &global.paddlers_icon, pos, small_icon_s);

        Ok(())
    }
}

// Draw a square image with a defined center and size
fn draw_at_center(canvas: &mut WebGLCanvas, image: &Image, center: Vector, size: f32) {
    let rect = Rectangle::new(center - Vector::new(size, size) / 2.0, (size, size));
    canvas.draw(&rect, image);
}
