use paddle::quicksilver_compat::*;
use paddle::*;
use wasm_bindgen::prelude::wasm_bindgen;

// Shaders are defined in separate files
pub const VERTEX_SHADER: &str = include_str!("vertex_shader.glsl");
pub const FRAGMENT_SHADER: &str = include_str!("fragment_shader.glsl");

const BACKGROUND: Color = Color {
    r: 0.15,
    g: 0.15,
    b: 0.15,
    a: 1.0,
};

#[wasm_bindgen]
pub fn start() {
    // Build configuration object to define all setting
    let config = PaddleConfig::default()
        .with_canvas_id("paddle-canvas-id")
        .with_background_color(BACKGROUND)
        .with_resolution((1920, 1080))
        .with_texture_config(
            TextureConfig::default()
            .without_filter()
        );

    // Initialize framework state and connect to browser window
    paddle::init(config).expect("Paddle initialization failed.");

    wasm_bindgen_futures::spawn_local(start_async());
}


// Load images and then start the game
async fn start_async() {
    let icon = Image::load("paddle_icon.svg").await.expect("Icon not found");
    let custom_rendering = None;
    let state = Graphics{ icon, custom_rendering };
    paddle::register_frame(Game {}, state, (0, 0));
}

struct Graphics {
    icon: Image,
    custom_rendering: Option<CustomShader>,
}
struct Game {}

impl paddle::Frame for Game {
    // This simple example does not need any shared state variables.
    type State = Graphics;
    // This defines the size of the frame (in game coordinates)
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;

    // Will get called ~60 times per second, or might be adapted to the screen refresh rate. (Browser will decide)
    fn draw(&mut self, graphics: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        if graphics.custom_rendering.is_none() {
            graphics.init_my_shader(canvas);
        }

        canvas.fit_display(10.0);
        
        // Draw a padded area with custom shader without touching what is outside that area.
        // But before we can do that, make sure the time uniform is up to date.
        canvas.update_uniform(graphics.custom_rendering.as_ref().unwrap().paint_render_pipeline(), "Time", &UniformValue::F32((timestamp / 1000.0) as f32));
        canvas.draw_ex(
            &Self::area().padded(50.0),
            graphics.custom_rendering.as_ref().unwrap(),
            Transform::IDENTITY,
            1
        );
        
        // Draw icon at the center, on top of custom shaded area
        let rect = Rectangle::new((820, 400), (280, 280));
        canvas.draw_ex(&rect, &graphics.icon, Transform::IDENTITY, 2);
    }
}



impl Graphics {
    fn init_my_shader(&mut self, canvas: &mut DisplayArea) {
        let vertex_shader = VERTEX_SHADER;
        let fragment_shader = FRAGMENT_SHADER;
        let vertex_descriptor = VertexDescriptor::new().with_pos();
        // .with("");

        let projection = canvas.full().webgl_transform();

        let uniform_values = &[("Projection", UniformValue::Matrix3fv(projection.as_slice())), ("Time", UniformValue::F32(0.5))];
        
        self.custom_rendering = Some(CustomShader::new(canvas.new_render_pipeline(
            vertex_shader,
            fragment_shader,
            vertex_descriptor,
            uniform_values
        ).expect("Loading shader failed")).with_color(Color::WHITE));
    }
}