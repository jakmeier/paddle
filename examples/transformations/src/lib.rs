//! Example to demonstrate the use of transformations when drawing images and geometrics.
//!
//! Using transformation can be a bit tricky for poeple (like me) who haven't used geometric math in some time.
//! Thus this example demonstrates visually what applying transformation in different orders produce.
//! (It also helps me to verify that the implemention does what it should even after major refactoring in the library)
//! 

use paddle::graphics::{Image, TextureConfig};
use paddle::quicksilver_compat::*;
use paddle::quicksilver_compat::geom::Triangle;
use paddle::{DisplayArea, PaddleConfig, Vector, Rectangle, Transform};
use wasm_bindgen::prelude::wasm_bindgen;

const SCREEN_W: f32 = 1920.0;
const SCREEN_H: f32 = 1080.0;

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

    // Quick version for now, awaiting futures one by one:
    let future = async move {
        let state = GlobalState {
            icon: icon.await.expect("loading icon failed"),
        };
        // Create our game state and register it
        paddle::register_frame(Game {}, state, (0, 0));
    };

    wasm_bindgen_futures::spawn_local(future);
}

struct Game {}
struct GlobalState {
    icon: Image,
}

impl paddle::Frame for Game {
    type State = GlobalState;
    const WIDTH: u32 = SCREEN_W as u32;
    const HEIGHT: u32 = SCREEN_H as u32;

    // Will get called ~60 times per second, or might be adapted to the screen refresh rate. (Browser will decide)
    fn draw(&mut self, global: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        canvas.fit_display(10.0);

        // White background
        canvas.fill(&Color::WHITE);

        // Z is used to define what is drawn behind what
        let base_z = 10;
        // The center of the drawing area
        let center = Self::size() / 2.0;
        // Base size of all objects drawn
        let s = 300.0;
        
        // Define a rotation transform that changes with the timestamp (60 degrees per second)
        let rotation = Transform::rotate(timestamp / 1000.0 * 60.0);


        
        /*
         * Left side
         */
        let center_left = Vector::new(center.x - Self::WIDTH as f32 / 6.0, center.y);
        let half_size = Vector::new(s,s)/2.0;
        let left_pos = center_left - half_size;
        let left_transform = Transform::translate(left_pos + half_size );

        // Mark area with a black non-moving rectangle.
        // Here we use draw_z without transformations
        canvas.draw_z(&Rectangle::new(left_pos, (s,s)), &Color::BLACK, base_z);
        
        // Now, using transformations.
        // The shape for drawing is defined as a rectangle around the origin (0,0). Otherwise, the rotation would not work as expected.
        let shape = Rectangle::new(-half_size, (s,s));

        // Draw an image that rotates behind
        canvas.draw_ex(&shape, &global.icon, left_transform *rotation, base_z + 1);
        // Behind all, draw a rectangle that rotates with twice the speed (rotation applied twice)
        canvas.draw_ex(&shape, &Color::INDIGO, left_transform *rotation * rotation, base_z - 1);
        // Behind all, draw an orange rectangle that is scaled to be larger and rotates in sync with the image
        canvas.draw_ex(&shape, &Color::ORANGE, left_transform *rotation * Transform::scale((1.5,1.5)), base_z - 2);
        
        /*
         * Right side
         * The same as on the left side but shifted to the right.
         * Without transformation, this could be done by changing the draw origin (left_area.pos + XY).
         * But with the transformation, we apply another translation as shown below.
         */
         let shift = Vector::new(Self::WIDTH as f32 /3.0, 0);
         let shift_transform = Transform::translate(shift);
         // Shifting drawn area
         {
            let mut right_area = shape;
            right_area.pos = left_pos + shift;
            // Mark area with a black non-moving rectangle
            canvas.draw_z(&right_area, &Color::BLACK, base_z);
         }
         // Now draw without using the shifted area, use transform instead.
         
         // Draw the same rectangles but shifted
         canvas.draw_ex(&shape, &Color::INDIGO, shift_transform * left_transform *rotation * rotation, base_z - 1);
         canvas.draw_ex(&shape, &Color::ORANGE, shift_transform * left_transform *rotation * Transform::scale((1.5,1.5)) , base_z - 2);
         // Draw the same image but shift it and flip it horizontally.
         // The flip is applied before the rotation, hence the rotation changes the direction.
         canvas.draw_ex(&shape, &global.icon, shift_transform * left_transform * Transform::horizontal_flip() * rotation , base_z + 1);
         // (This would also flip the image but keep the rotation in the same orientation)
         //  canvas.draw_ex(&shape, &global.icon, shift_transform * left_transform * rotation * Transform::horizontal_flip() , base_z + 1);
         
         // What happens if the translation is applied at the right instead? (Observe the red Rectangle)
         canvas.draw_ex(&shape, &Color::RED, rotation * shift_transform * left_transform, base_z - 1);
         // What if we do both? (observe the red Triangle)
         let triangle = Triangle::new( center_left + Vector::new(0,s), center_left + Vector::new(-s/2.0,0), center_left + Vector::new(s/2.0,0) );
         canvas.draw_ex(&triangle, &Color::RED, shift_transform * rotation * shift_transform, base_z - 1);


    }
}
