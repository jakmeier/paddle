//! This example uses multiple frames and user interactions.
//! It should demonstrate how to easily build a UI and keep it separate from
//! the main area of the game, while allowing for interactions in between.

use paddle::quicksilver_compat::*;
use paddle::{DisplayArea, PaddleConfig};
use wasm_bindgen::prelude::wasm_bindgen;

const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const BLACK: Color = Color::new(0.0, 0.0, 0.0);
const RED_CRAYOLA: Color = Color::new(0.929, 0.145, 0.306);
const NAPLES_YELLOW: Color = Color::new(0.976, 0.863, 0.361);
const TEA_GREEN: Color = Color::new(0.008, 0.918, 0.741);
const OXFORD_BLUE: Color = Color::new(0.004, 0.098, 0.212);
const BLACK_CORAL: Color = Color::new(0.275, 0.325, 0.384);

#[wasm_bindgen]
pub fn start() {
    // Build configuration object to define all settings
    let config = PaddleConfig::default()
        .with_canvas_id("paddle-canvas-id")
        .with_background_color(BLACK)
        .with_resolution((1280, 720));

    // Start game engine
    paddle::init(config).expect("Paddle initialization failed.");

    // Create our game state and register it
    let mut state = SharedState::new(OXFORD_BLUE);
    state.add_rectangle((20, 20), (50, 50), RED_CRAYOLA);
    // Toolbar on the left
    paddle::register_frame(Toolbar::new(260, 720), state, (0, 0), (260, 720));
    // Area to draw on, with a shifted root position. (270|10)
    // All coordinates inside will be relative to that root.
    paddle::register_frame_no_state(Paper::default(), (270, 10), (1000, 700));
}

#[derive(Default)]
struct Paper {
    first_click: Option<Vector>,
}

struct SharedState {
    drawn_objects: Vec<(Rectangle, Color)>,
    selected_color: Color,
}

impl paddle::Frame for Paper {
    type State = SharedState;
    fn draw(&mut self, state: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        // Adapt canvas size to viewport on every frame
        canvas.fit_display(10.0);

        let rect = Rectangle::new((0, 0), (1000, 700));
        canvas.draw(&rect, WHITE);

        for (rect, col) in &state.drawn_objects {
            canvas.draw(rect, *col);
        }
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        if let Some(first_click) = self.first_click {
            let w = (first_click.x - pos.0 as f32).abs();
            let h = (first_click.y - pos.1 as f32).abs();
            let x = first_click.x.min(pos.0 as f32);
            let y = first_click.y.min(pos.1 as f32);
            state.add_rectangle((x, y), (w, h), state.selected_color);
        } else {
            self.first_click = Some(pos.into());
        }
    }
    fn right_click(&mut self, _state: &mut Self::State, _pos: (i32, i32)) {
        self.first_click = None;
    }
}
impl SharedState {
    fn new(selected_color: Color) -> Self {
        Self {
            drawn_objects: vec![],
            selected_color,
        }
    }

    fn add_rectangle(&mut self, pos: impl Into<Vector>, size: impl Into<Vector>, col: Color) {
        self.drawn_objects.push((Rectangle::new(pos, size), col));
    }
}

struct Toolbar {
    ui_elements: Vec<(Rectangle, Color)>,
}
impl Toolbar {
    fn new(frame_width: usize, frame_height: usize) -> Self {
        const W: usize = 100;
        const MARGIN: usize = 20;
        let w = frame_width - (2 * MARGIN);
        let d = w - (2 * W);
        let mut ui_elements = Vec::new();
        // Place the first row of UI rectangles (to pick color)
        let rect_left = Rectangle::new((MARGIN, MARGIN), (W, W));
        let rect_right = Rectangle::new((frame_width - MARGIN - W, MARGIN), (W, W));
        // add all colors, row by row
        let colors = [WHITE, BLACK, RED_CRAYOLA, NAPLES_YELLOW, TEA_GREEN, OXFORD_BLUE];
        for (i, col) in colors.iter().enumerate() {
            let mut rect;
            if i % 2 == 0 {
                rect = rect_left;
            } else {
                rect = rect_right;
            }
            rect.pos.y += ((i / 2) * (W + d)) as f32;

            ui_elements.push((rect, *col));
        }

        Self { ui_elements }
    }
}
impl paddle::Frame for Toolbar {
    type State = SharedState;

    fn draw(&mut self, _state: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        let rect = Rectangle::new((0, 0), (260, 720));
        canvas.draw(&rect, BLACK_CORAL);
        for (area, col) in &self.ui_elements {
            canvas.draw(area, *col);
        }
    }
    fn left_click(&mut self, state: &mut Self::State, pos: (i32, i32)) {
        for (area, col) in &self.ui_elements {
            if area.contains(pos) {
                state.selected_color = *col;
            }
        }
    }
}
