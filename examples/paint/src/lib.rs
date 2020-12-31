//! This example uses multiple frames and user interactions.
//! It should demonstrate how to easily build a UI and keep it separate from
//! the main area of the game, while allowing for interactions in between.

use paddle::quicksilver_compat::*;
use paddle::{
    DisplayArea, Frame, KeyEvent, PaddleConfig, PointerEvent, PointerEventType, Rectangle, Vector,
};
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
    state.add_rectangle(Rectangle::new((20, 20), (50, 50)), RED_CRAYOLA);
    // Toolbar on the left
    paddle::register_frame(Toolbar::new(), state, (0, 0));
    // Area to draw on, with a shifted root position. (270|10)
    // All coordinates inside will be relative to that root.
    paddle::register_frame_no_state(Paper::default(), (270, 10));
}

#[derive(Default)]
struct Paper {
    first_click: Option<Vector>,
    mouse_pos: Option<Vector>,
}

struct SharedState {
    drawn_objects: Vec<(Rectangle, Color)>,
    selected_color: Color,
}

impl Frame for Paper {
    type State = SharedState;
    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 700;
    fn draw(&mut self, state: &mut Self::State, canvas: &mut DisplayArea, _timestamp: f64) {
        // Adapt canvas size to viewport on every frame
        canvas.fit_display(10.0);

        // Paint background of frame white
        canvas.fill(WHITE);

        for (rect, col) in &state.drawn_objects {
            canvas.draw(rect, *col);
        }

        if let (Some(pos1), Some(pos2)) = (self.first_click, self.mouse_pos) {
            let rect = rectangle_from_two_points(pos1, pos2);
            canvas.draw(&rect, state.selected_color);
        }
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        match event {
            PointerEvent(PointerEventType::PrimaryClick, pos) => {
                if let Some(first_click) = self.first_click {
                    let rect = rectangle_from_two_points(first_click, pos);
                    state.add_rectangle(rect, state.selected_color);
                    self.first_click = None;
                } else {
                    self.first_click = Some(pos);
                }
            }
            PointerEvent(PointerEventType::SecondaryClick, _pos) => {
                self.first_click = None;
            }
            PointerEvent(PointerEventType::Move, pos) => {
                self.mouse_pos = Some(pos);
            }
            _ => {}
        }
    }
}
impl SharedState {
    fn new(selected_color: Color) -> Self {
        Self {
            drawn_objects: vec![],
            selected_color,
        }
    }

    fn add_rectangle(&mut self, rect: Rectangle, col: Color) {
        self.drawn_objects.push((rect, col));
    }
}

struct Toolbar {
    ui_elements: Vec<(Rectangle, Color)>,
}
impl Toolbar {
    fn new() -> Self {
        const W: u32 = 100;
        const MARGIN: u32 = 20;
        let w = Self::WIDTH - (2 * MARGIN);
        let d = w - (2 * W);
        let mut ui_elements = Vec::new();
        // Place the first row of UI rectangles (to pick color)
        let rect_left = Rectangle::new((MARGIN, MARGIN), (W, W));
        let rect_right = Rectangle::new((Self::WIDTH - MARGIN - W, MARGIN), (W, W));
        // add all colors, row by row
        let colors = [
            WHITE,
            BLACK,
            RED_CRAYOLA,
            NAPLES_YELLOW,
            TEA_GREEN,
            OXFORD_BLUE,
        ];
        for (i, col) in colors.iter().enumerate() {
            let mut rect;
            if i % 2 == 0 {
                rect = rect_left;
            } else {
                rect = rect_right;
            }
            rect.pos.y += ((i as u32 / 2) * (W + d)) as f32;

            ui_elements.push((rect, *col));
        }

        Self { ui_elements }
    }
}
impl Frame for Toolbar {
    type State = SharedState;
    const WIDTH: u32 = 260;
    const HEIGHT: u32 = 720;

    fn draw(&mut self, _state: &mut Self::State, frame_display: &mut DisplayArea, _timestamp: f64) {
        frame_display.fill(BLACK_CORAL);
        for (area, col) in &self.ui_elements {
            frame_display.draw(area, *col);
        }
    }
    fn pointer(&mut self, state: &mut Self::State, event: PointerEvent) {
        match event {
            PointerEvent(PointerEventType::PrimaryClick, pos) => {
                for (area, col) in &self.ui_elements {
                    if area.contains(pos) {
                        state.selected_color = *col;
                    }
                }
            }
            _ => {}
        }
    }
    fn key(&mut self, state: &mut Self::State, key_event: KeyEvent) {
        match key_event {
            KeyEvent(KeyEventType::KeyDown, Key::Delete)
            | KeyEvent(KeyEventType::KeyDown, Key::Backspace) => {
                state.drawn_objects.pop();
            }
            _ => {}
        }
    }
}

fn rectangle_from_two_points(pos1: Vector, pos2: Vector) -> Rectangle {
    let w = (pos1.x - pos2.x).abs();
    let h = (pos1.y - pos2.y).abs();
    let x = pos1.x.min(pos2.x);
    let y = pos1.y.min(pos2.y);
    Rectangle::new((x, y), (w, h))
}
