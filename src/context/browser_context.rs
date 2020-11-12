use crate::graphics::ImageLoader;
use crate::quicksilver_compat::Vector;
use crate::web_integration::ThreadHandler;
use crate::*;
use web_sys::HtmlCanvasElement;

pub(crate) struct BrowserContext {
    canvas: WebGLCanvas,
    #[allow(dead_code)]
    draw_handle: ThreadHandler,
    #[allow(dead_code)]
    update_handle: ThreadHandler,
}

pub struct BrowserConfig {
    pub canvas: HtmlCanvasElement,
    pub pixels: Vector,
}

impl BrowserContext {
    pub(super) fn new(config: BrowserConfig) -> PaddleResult<Self> {
        let draw_handle = start_drawing()?;
        let update_handle = start_updating(10)?;

        let canvas = WebGLCanvas::new(config.canvas, config.pixels)?;

        // For binding textures as they arrive
        ImageLoader::register(canvas.clone_webgl());

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
