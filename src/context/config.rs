use crate::graphics::TextureConfig;
use crate::quicksilver_compat::Vector;
use crate::BrowserConfig;
use crate::CanvasConfig;
use web_sys::HtmlCanvasElement;

#[derive(Default)]
pub struct PaddleConfig {
    pub browser: BrowserConfig,
    pub enable_text_board: bool,
}

impl PaddleConfig {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_canvas_id(mut self, id: &'static str) -> Self {
        self.browser.canvas = CanvasConfig::HtmlId(id);
        self
    }
    pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
        self.browser.canvas = CanvasConfig::HtmlElement(canvas);
        self
    }
    pub fn with_resolution(mut self, pixels: impl Into<Vector>) -> Self {
        self.browser.pixels = pixels.into();
        self
    }
    pub fn with_texture_config(mut self, texture_config: TextureConfig) -> Self {
        self.browser.texture_config = texture_config;
        self
    }
}
