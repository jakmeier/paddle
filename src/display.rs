//! Parent module around code that puts stuff on the screen.
//!
//! This covers basics things like working with an HTML canvas and some WebGL interfacing.
//! Displaying text is currently also under this parent module.

mod canvas;
mod display_area;
mod gpu;
mod text;

pub use canvas::*;
pub use display_area::*;
pub use text::*;

use crate::quicksilver_compat::Vector;
use crate::*;
use crate::{graphics::ImageLoader, quicksilver_compat::Color};
use crate::{graphics::TextureConfig, quicksilver_compat::Mesh};
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

/// An object to manage the *full* display area for your game inside the browser.
pub struct Display {
    /// Position relative to browser page (in browser coordinates)
    browser_region: Rectangle,
    /// Game / World coordinates. Used to refer to game objects and UI elements independently of resolution or size in the browser.
    game_coordinates: Vector,
    /// A canvas with WebGL capabilities (covering the full display area)
    canvas: WebGLCanvas,
    /// Screen background color. A clear to this color is invoked every frame.
    background_color: Option<Color>,
}

pub struct DisplayConfig {
    pub canvas: CanvasConfig,
    pub pixels: Vector,
    pub texture_config: TextureConfig,
    pub update_delay_ms: i32,
    pub background: Option<Color>,
}
impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            canvas: CanvasConfig::HtmlId("paddle-canvas"),
            pixels: Vector::new(1280, 720),
            update_delay_ms: 8,
            texture_config: Default::default(),
            background: None,
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

        let background_color = config.background;

        Ok(Self {
            canvas,
            browser_region,
            game_coordinates,
            background_color,
        })
    }
    pub(crate) fn canvas_mut(&mut self) -> &mut WebGLCanvas {
        &mut self.canvas
    }

    /// Position relative to browser page and size in browser pixels
    pub fn browser_region(&self) -> Rectangle {
        self.browser_region
    }

    pub fn clear(&mut self) {
        if let Some(col) = self.background_color {
            self.canvas.clear(col);
        }
    }

    /// Transforms from coordinates used inside the game (aka world coordinates) to browser coordinates (as used by e.g. CSS pixels)
    pub fn game_to_browser_coordinates(&self) -> Transform {
        Transform::scale(
            self.browser_region
                .size
                .times(self.game_coordinates.recip()),
        ) * Transform::translate(self.browser_region.pos)
    }

    /// Gives result for x axis (assuming y is the same)
    pub fn browser_to_game_pixel_ratio(&self) -> f32 {
        self.browser_region.width() / self.game_coordinates.x
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

    pub(crate) fn mesh(&mut self) -> &mut Mesh {
        self.canvas.mesh()
    }

    // Insert triangles to buffer without modifications.
    pub fn draw_triangles(&mut self, mesh: &Mesh) {
        let n = self.mesh().vertices.len() as u32;
        self.mesh().vertices.extend_from_slice(&mesh.vertices);
        self.mesh()
            .triangles
            .extend(mesh.triangles.iter().cloned().map(|mut t| {
                t.indices[0] += n;
                t.indices[1] += n;
                t.indices[2] += n;
                t
            }));
    }
    // Insert triangles to buffer after applying a transform modifications.
    pub fn draw_triangles_ex(&mut self, mesh: &Mesh, t: Transform) {
        let n = self.mesh().vertices.len() as u32;
        for mut vertex in mesh.vertices.iter().cloned() {
            vertex.pos = t * vertex.pos;
            vertex.tex_pos = vertex.tex_pos.map(|v| t * v);
            self.mesh().vertices.push(vertex);
        }
        self.mesh()
            .triangles
            .extend(mesh.triangles.iter().cloned().map(|mut t| {
                t.indices[0] += n;
                t.indices[1] += n;
                t.indices[2] += n;
                t
            }));
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
