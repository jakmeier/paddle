use paddle::graphics::TextureConfig;
use paddle::quicksilver_compat::*;
use paddle::{
    AssetBundle, AssetLoadingTracker, DisplayArea, ImageDesc, PaddleConfig, Rectangle, Vector,
};
use wasm_bindgen::prelude::wasm_bindgen;

const SCREEN_W: f32 = 1920.0;
const SCREEN_H: f32 = 1080.0;

// Describe images by their path. The format doesn't matter, as long as a browser can display it.
const IMG_BACKGROUND: ImageDesc = ImageDesc::new("background.png");
const IMG_PADDLE_ICON: ImageDesc = ImageDesc::new("paddle_icon.svg");
const IMG_PADDLERS_ICON: ImageDesc = ImageDesc::new("paddlers.svg");

struct Game {
    state: GameState,
    load_tracker: AssetLoadingTracker,
}
enum GameState {
    Loading,
    Ready,
}

// This is the equivalent of `fn main()` but for the browser. It gets called once when the web page loads.
#[wasm_bindgen]
pub fn start() {
    // Build configuration object to define all settings
    let texture_config = TextureConfig::default().without_filter().with_rgba();
    let config = PaddleConfig::default()
        .with_canvas_id("paddle-canvas-id")
        .with_resolution((SCREEN_W, SCREEN_H))
        .with_texture_config(texture_config);

    // Start game engine
    paddle::init(config).expect("Paddle initialization failed.");

    // Load images
    let mut asset_bundle = AssetBundle::new();
    asset_bundle.add_images(&[IMG_PADDLE_ICON, IMG_PADDLERS_ICON, IMG_BACKGROUND]);
    let load_tracker = asset_bundle.load();

    // Images are now loaded asynchronously. Progress can be checked on load_tracker.

    // Start game
    let game = Game {
        state: GameState::Loading,
        load_tracker,
    };
    paddle::register_frame(game, GlobalState {}, (0, 0));
}

// A type used for state shared among frames
struct GlobalState {
    // add fields here if you want to shared state
}

// This example has a single frame with the entire game in it.
// In the implementation of `paddle::Frame` for `Game`, we define how the frame looks.
impl paddle::Frame for Game {
    // A global state object is shared among all frames. Not very interesting with only a single frame.
    type State = GlobalState;
    // Define the size of the frame in pixels.
    const WIDTH: u32 = SCREEN_W as u32;
    const HEIGHT: u32 = SCREEN_H as u32;

    // Draw the game in this function.
    // In this example, we forward most of the drawing to other functions, depending on the game state.
    fn draw(&mut self, _global: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        // Adjust screen size every frame to keep it fitted into the window size.
        canvas.fit_display(10.0);

        match self.state {
            GameState::Loading => {
                self.draw_loading(canvas);
            }
            GameState::Ready => {
                self.draw_ready(canvas, timestamp);
            }
        }
    }

    fn update(&mut self, _global: &mut Self::State) {
        if self.load_tracker.progress() == 1.0 {
            self.state = GameState::Ready;
        }
    }
}
impl Game {
    // While loading, display a progress bar
    fn draw_loading(&mut self, canvas: &mut DisplayArea) {
        let bar_area = Rectangle::new(
            (100.0, SCREEN_H as f32 / 2.0 + 50.0),
            (SCREEN_W as f32 * 0.9, 100.0),
        );
        let progress = self.load_tracker.progress();

        let my_grey = Color::new(0.15, 0.15, 0.2);
        let my_silver = Color::new(0.5, 0.5, 0.55);

        canvas.fill(&my_grey);
        canvas.draw(&bar_area, &Color::WHITE);
        let mut bar = bar_area.padded(3.0);
        canvas.draw(&bar, &Color::BLACK);
        bar.size.x *= progress;
        canvas.draw(&bar, &my_silver);
    }

    // Once loaded, we draw three images.
    fn draw_ready(&mut self, canvas: &mut DisplayArea, timestamp: f64) {
        // Background image filling the screen
        canvas.fill(&IMG_BACKGROUND);

        // Large icon in the center
        let icon_s = 500.0;
        let center = Vector::new(SCREEN_W / 2.0, SCREEN_H / 2.0);
        draw_at_center(canvas, &IMG_PADDLE_ICON, center, icon_s);

        // Smaller icons orbit the large icon with a certain radius.
        let small_icon_s = icon_s / 1.618f32.powi(2);
        let r = 450.0;

        // We make use of the millisecond timestamp and trigonometric formulas to calculate the position on a circle.
        let period = timestamp / 3000.0;
        let direction = Vector::new(period.cos(), period.sin());
        let pos = center + direction * r;
        draw_at_center(canvas, &IMG_PADDLE_ICON, pos, small_icon_s);

        let period = timestamp / 3000.0 + std::f64::consts::PI;
        let direction = Vector::new(period.cos(), period.sin());
        let pos = center + direction * r;
        draw_at_center(canvas, &IMG_PADDLERS_ICON, pos, small_icon_s);
    }
}

// Small helper function to draw a square image with a defined center and size
fn draw_at_center(canvas: &mut DisplayArea, image: &ImageDesc, center: Vector, size: f32) {
    let rect = Rectangle::new(center - Vector::new(size, size) / 2.0, (size, size));
    canvas.draw(&rect, image);
}
