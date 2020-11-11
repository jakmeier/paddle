use paddle::quicksilver_compat::*;
use paddle::WebGLCanvas;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn start() {
    paddle::TextBoard::init();
    let mut canvas = WebGLCanvas::from_canvas_id("paddle-canvas-id", 960, 540)
        .expect("Failed to initialize WebGL");
    canvas.fit_to_screen(0.0).expect("Failed fitting to screen");

    // Starting a requestAnimationFrame loop
    let handle = paddle::web_integration::start_drawing_thread(move |timestamp| {
        draw_frame(&mut canvas, timestamp)
    })
    .expect("Creating thread failed");

    // Usually the handle would be stored somewhere and it can be used to stop the thread at will.
    // When the handle is dropped, the drawing thread will stop and its memory is released.
    //
    // Here, memory is leaked to avoid the handle from being dropped immediately (and the thread deleted).
    // This is fine, we never intended to stop the drawing thread anyway.
    // Having no intention of ever releasing memory is as good as simply "leaking" it.
    std::mem::forget(handle);
}

// Will get called ~60 times per second, or might be adapted to the screen refresh rate. (Browser will decide)
fn draw_frame(canvas: &mut WebGLCanvas, timestamp: f64) {
    
    // Adapt canvas size to viewport on every frame
    canvas.fit_to_screen(10.0).expect("Failed fitting to screen");

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

    // Finish drawing
    canvas.flush().expect("WebGL flush failed");
}
