use paddle::quicksilver_compat::*;
use paddle::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlTextAreaElement;
use FitStrategy::*;

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
        .with_texture_config(TextureConfig::default().without_filter())
        .with_text_board(Rectangle::new((510, 340), (900, 400)));

    // Initialize framework state and connect to browser window
    paddle::init(config).expect("Paddle initialization failed.");

    wasm_bindgen_futures::spawn_local(start_async());
}

// Load images and then start the game
async fn start_async() {
    let texture = Image::load("water.png").await.expect("Icon not found");
    let custom_rendering = None;
    let state = Graphics {
        custom_rendering,
        texture,
    };
    let text = || FloatingText::try_default().expect("Text creation failed");
    let prompt = Prompt {
        vert_html: None,
        frag_html: None,
        button_area: Rectangle::new(
            (100, Prompt::HEIGHT / 10 * 9 - 100),
            ((Prompt::WIDTH - 200), 100),
        ),
        button_text: text(),
        new_shaders: None,
    };
    let preview = Preview {
        preview_text: text(),
    };
    paddle::register_frame(prompt, state, (0, 0));
    paddle::register_frame_no_state(preview, (960, 0));
}

struct Graphics {
    custom_rendering: Option<CustomShader>,
    texture: Image,
}
struct Prompt {
    vert_html: Option<HtmlTextAreaElement>,
    frag_html: Option<HtmlTextAreaElement>,
    button_area: Rectangle,
    button_text: FloatingText,
    new_shaders: Option<(String, String)>,
}
struct Preview {
    preview_text: FloatingText,
}

impl paddle::Frame for Prompt {
    type State = Graphics;
    const WIDTH: u32 = 960;
    const HEIGHT: u32 = 1080;

    fn draw(&mut self, graphics: &mut Self::State, canvas: &mut DisplayArea, _timestamp: f64) {
        if self.vert_html.is_none() {
            self.init_html(canvas);
        }
        if let Some((vertex, fragment)) = self.new_shaders.take() {
            if let Err(msg) = graphics.init_my_shader(canvas, &vertex, &fragment) {
                TextBoard::display_error_message(msg.text).unwrap();
            }
        }
        canvas.draw(&self.button_area, &Color::from_hex("55FF33"));
        self.button_text
            .write(canvas, &self.button_area, 1, Center, "Load Shader")
            .unwrap();
    }

    fn pointer(&mut self, _state: &mut Self::State, event: PointerEvent) {
        if event.event_type() == PointerEventType::PrimaryClick
            && self.button_area.contains(event.pos())
        {
            self.new_shaders = Some((
                self.vert_html.as_ref().unwrap().value(),
                self.frag_html.as_ref().unwrap().value(),
            ));
        }
    }
}

impl Prompt {
    fn init_html(&mut self, canvas: &mut DisplayArea) {
        let doc = web_sys::window().unwrap().document().unwrap();
        let root = doc.create_element("div").unwrap();
        let vert_textarea = new_edit_field(VERTEX_SHADER);
        let frag_textarea = new_edit_field(FRAGMENT_SHADER);
        root.append_child(&vert_textarea).unwrap();
        root.append_child(&frag_textarea).unwrap();
        canvas.add_html(root);
        self.vert_html = Some(vert_textarea);
        self.frag_html = Some(frag_textarea);
    }
}

fn new_edit_field(text: &str) -> HtmlTextAreaElement {
    paddle::html::text_area(text, 72, 20, "prompt")
}

impl paddle::Frame for Preview {
    type State = Graphics;
    const WIDTH: u32 = 960;
    const HEIGHT: u32 = 1080;

    fn draw(&mut self, graphics: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        canvas.fit_display(10.0);

        if graphics.custom_rendering.is_none() {
            self.preview_text.show().expect("showing text failed");
            let area = Rectangle::new((0, (Self::HEIGHT - 100) / 2), (Self::WIDTH, 100));
            let text = "Please load a shader on the left.";
            self.preview_text
                .write(canvas, &area, 1, Center, text)
                .expect("writing failed");
            canvas.draw(&area.padded(20.0), &Color::WHITE);
        } else {
            self.preview_text.hide().expect("hiding text failed");
            draw_preview(graphics, canvas, timestamp);
        }
    }
}

// Draw a padded area with custom shader without touching what is outside that area.
// But before we can do that, make sure the time uniform is up to date.
fn draw_preview(graphics: &mut Graphics, canvas: &mut DisplayArea, timestamp: f64) {
    let render_pipeline = Paint::paint_render_pipeline(graphics.custom_rendering.as_ref().unwrap());
    canvas.update_uniform(
        render_pipeline,
        "Time",
        &UniformValue::F32((timestamp / 1000.0) as f32),
    );
    canvas.draw_ex(
        &Preview::area().padded(50.0),
        graphics.custom_rendering.as_ref().unwrap(),
        Transform::translate(Preview::area().center())
            * Transform::rotate(45)
            * Transform::translate(-Preview::area().center()),
        1,
    );
}

impl Graphics {
    fn init_my_shader(
        &mut self,
        canvas: &mut DisplayArea,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<(), ErrorMessage> {
        let vertex_descriptor = VertexDescriptor::new().with_pos().with_tex();
        let projection = canvas.full().webgl_transform();
        let uniform_values = &[
            ("Projection", projection.into()),
            ("Time", UniformValue::F32(0.5)),
        ];
        self.custom_rendering = Some(
            CustomShader::new(canvas.new_render_pipeline(
                vertex_shader,
                fragment_shader,
                vertex_descriptor,
                uniform_values,
            )?)
            .with_image(self.texture.clone()),
        );
        Ok(())
    }
}
