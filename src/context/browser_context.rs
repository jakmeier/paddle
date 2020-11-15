use crate::graphics::ImageLoader;
use crate::graphics::TextureConfig;
use crate::quicksilver_compat::Vector;
use crate::web_integration::ThreadHandler;
use crate::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub(crate) struct BrowserContext {
    canvas: WebGLCanvas,
    #[allow(dead_code)]
    draw_handle: ThreadHandler,
    #[allow(dead_code)]
    update_handle: ThreadHandler,
}

pub struct BrowserConfig {
    pub canvas: CanvasConfig,
    pub pixels: Vector,
    pub texture_config: TextureConfig,
    pub update_delay_ms: i32,
}
impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            canvas: CanvasConfig::HtmlId("paddle-canvas"),
            pixels: Vector::new(1280, 720),
            update_delay_ms: 8,
            texture_config: Default::default(),
        }
    }
}

pub enum CanvasConfig {
    HtmlId(&'static str),
    HtmlElement(HtmlCanvasElement),
}

impl BrowserContext {
    pub(super) fn new(config: BrowserConfig) -> PaddleResult<Self> {
        let draw_handle = start_drawing()?;
        let update_handle = start_updating(config.update_delay_ms)?;

        let canvas = match config.canvas {
            CanvasConfig::HtmlElement(el) => el,
            CanvasConfig::HtmlId(id) => canvas_by_id(id)?,
        };

        div::init_ex(
            Some("game-root"),
            (0, 0),
            Some((config.pixels.x as u32, config.pixels.y as u32)),
        )
        .expect("Div initialization failed");

        let canvas = WebGLCanvas::new(canvas, config.pixels)?;

        // For binding textures as they arrive
        ImageLoader::register(canvas.clone_webgl(), config.texture_config);

        Ok(Self {
            canvas,
            draw_handle,
            update_handle,
        })
    }
    pub(crate) fn canvas_mut(&mut self) -> &mut WebGLCanvas {
        &mut self.canvas
    }
}

pub fn canvas_by_id(id: &str) -> PaddleResult<HtmlCanvasElement> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(id)
        .ok_or_else(|| ErrorMessage::technical(format!("No canvas with id {}", id)))?;
    canvas.dyn_into::<HtmlCanvasElement>().map_err(|e| {
        ErrorMessage::technical(format!(
            "Not a canvas. Err: {}",
            e.to_string().as_string().unwrap()
        ))
    })
}
