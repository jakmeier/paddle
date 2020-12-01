// TODO: Restructure file hierarchy around display/canvas/graphics/frame
use crate::graphics::ImageLoader;
use crate::graphics::TextureConfig;
use crate::quicksilver_compat::Vector;
use crate::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

/// An object to manage the *display area* for your game inside the browser.
pub struct Display {
    /// Position relative to browser page (in browser coordinates)
    browser_region: Rectangle,
    /// A canvas covering the full display area with WebGL capabilities
    canvas: WebGLCanvas,
    /// Game / World coordinates. Used to refer to game objects and UI elements independently of resolution or size in the browser.
    game_coordinates: Vector,
}

pub struct DisplayConfig {
    pub canvas: CanvasConfig,
    pub pixels: Vector,
    pub texture_config: TextureConfig,
    pub update_delay_ms: i32,
}
impl Default for DisplayConfig {
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

use crate::quicksilver_compat::Rectangle;
use crate::quicksilver_compat::Transform;
use crate::{NutsCheck, PaddleResult};

impl Display {
    pub(super) fn new(config: DisplayConfig) -> PaddleResult<Self> {
        let draw_handle = start_drawing()?;
        let update_handle = start_updating(config.update_delay_ms)?;

        let canvas = match config.canvas {
            CanvasConfig::HtmlElement(el) => el,
            CanvasConfig::HtmlId(id) => canvas_by_id(id)?,
        };

        let canvas = WebGLCanvas::new(canvas, config.pixels)?;
        let browser_region = find_browser_region(canvas.html_element())?;
        div::init_ex(
            Some("game-root"),
            (browser_region.x() as u32, browser_region.y() as u32),
            Some((config.pixels.x as u32, config.pixels.y as u32)),
        )
        .expect("Div initialization failed");

        // For binding textures as they arrive
        ImageLoader::register(canvas.clone_webgl(), config.texture_config);

        // For now the only option is game_coordinates = pixels
        let game_coordinates = config.pixels;

        Ok(Self {
            canvas,
            browser_region,
            game_coordinates,
        })
    }
    pub(crate) fn canvas_mut(&mut self) -> &mut WebGLCanvas {
        &mut self.canvas
    }

    /// Position relative to browser page and size in browser pixels
    pub fn browser_region(&self) -> Rectangle {
        self.browser_region
    }

    /// Transforms from coordinates used inside the game (aka world coordinates) to browser coordinates (as used by e.g. CSS pixels)
    pub fn game_to_browser_coordinates(&self) -> Transform {
        Transform::scale(
            self.browser_region
                .size
                .times(self.game_coordinates.recip()),
        ) * Transform::translate(self.browser_region.pos)
    }

    ///Get the unprojection matrix according to the View
    pub fn unproject(&self) -> Transform {
        todo!()
        // Transform::scale(self.browser_region().size()) * self.view.normalize
    }

    ///Get the projection matrix according to the View
    pub fn project(&self) -> Transform {
        self.unproject().inverse()
    }

    /// Fixed to 16:9 ratio for now
    pub fn fit_to_visible_area(&mut self, margin: f64) -> PaddleResult<()> {
        let web_window = web_sys::window().unwrap();

        let w = web_window
            .inner_width()
            .map_err(JsError::from_js_value)?
            .as_f64()
            .unwrap();
        let h = web_window
            .inner_height()
            .map_err(JsError::from_js_value)?
            .as_f64()
            .unwrap();

        let (w, h) = scale_16_to_9(
            w - self.browser_region.x() as f64 - margin,
            h - self.browser_region.y() as f64 - margin,
        );

        self.canvas.set_size((w as f32, h as f32));

        // Resizing might change position (How exactly can be completely unpredictable due to CSS, media-queries etc.)
        self.update_browser_region();

        div::reposition(
            self.browser_region.pos.x as u32,
            self.browser_region.pos.y as u32,
        )?;
        div::resize(
            self.browser_region.size.x as u32,
            self.browser_region.size.y as u32,
        )?;
        Ok(())
    }

    fn update_browser_region(&mut self) {
        if let Some(br) = find_browser_region(self.canvas.html_element()).nuts_check() {
            self.browser_region = br;
        }
    }
}

fn find_browser_region(canvas: &HtmlCanvasElement) -> PaddleResult<Rectangle> {
    let web_window = web_sys::window().unwrap();
    let dom_rect = canvas.get_bounding_client_rect();

    let page_x_offset = web_window.page_x_offset().map_err(JsError::from_js_value)?;
    let page_y_offset = web_window.page_y_offset().map_err(JsError::from_js_value)?;
    let x = dom_rect.x() + page_x_offset;
    let y = dom_rect.y() + page_y_offset;
    let w = dom_rect.width();
    let h = dom_rect.height();
    let browser_region = Rectangle::new((x as f32, y as f32), (w as f32, h as f32));
    Ok(browser_region)
}

fn canvas_by_id(id: &str) -> PaddleResult<HtmlCanvasElement> {
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

fn scale_16_to_9(w: f64, h: f64) -> (f64, f64) {
    if w * 9.0 > h * 16.0 {
        (h * 16.0 / 9.0, h)
    } else {
        (w, w * 9.0 / 16.0)
    }
}
