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
use div::PaneHandle;
pub use text::*;

use crate::quicksilver_compat::Vector;
use crate::*;
use crate::{
    graphics::ImageLoader, graphics::TextureConfig, quicksilver_compat::Color,
    quicksilver_compat::Mesh,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlCanvasElement};

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
    /// Div element covering the full screen. (could be used for html elements outside of any frames)
    div: PaneHandle,
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

        // For now the only option is game_coordinates = pixels
        let game_coordinates = config.pixels;

        let canvas = WebGLCanvas::new(canvas, config.pixels)?;
        // Browser region is relative to window and needs to be known to handle input
        let browser_region = find_browser_region(canvas.html_element())?;

        // Initialize with game coordinates, which allows using them again for later calls
        let size = (game_coordinates.x as u32, game_coordinates.y as u32);
        div::init_ex(Some("game-root"), (0, 0), Some(size)).expect("Div initialization failed");

        div::resize(
            browser_region.width() as u32,
            browser_region.height() as u32,
        )
        .expect("Div initialization failed");

        // For binding textures as they arrive
        ImageLoader::register(canvas.clone_webgl(), config.texture_config);

        let background_color = config.background;

        let div = div::new_styled_pane::<_, _, &'static str, _, _>(
            0,
            0,
            size.0,
            size.1,
            "",
            &[],
            &[("pointer-events", "None")],
        )?;

        Ok(Self {
            canvas,
            browser_region,
            game_coordinates,
            background_color,
            div,
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
        self.adjust_display()?;
        Ok(())
    }
    /// Look up size and position of canvas in the browser and update input and drawing transformations accordingly.
    /// This should be called after the canvas size has changed, for example when the window is resized.
    /// When calling `fit_to_visible_area()`, the display is adjusted automatically (no need to call `adjust_display()` manually).
    pub fn adjust_display(&mut self) -> PaddleResult<()> {
        self.update_browser_region();

        let (x, y) = self.div_offset()?;
        div::reposition(x, y)?;
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
    /// Offset to ancestor with respect to which absolute positioned elements will be placed. (in browser coordinates)
    fn div_offset(&self) -> PaddleResult<(u32, u32)> {
        find_div_offset(
            self.canvas.html_element().clone().into(),
            &self.browser_region,
        )
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
fn find_div_offset(canvas: Element, browser_region: &Rectangle) -> PaddleResult<(u32, u32)> {
    let npa = nearest_positioned_ancestor(canvas).map_err(JsError::from_js_value)?;
    let npa_rect = npa.get_bounding_client_rect();
    let npa_pos = Vector::new(npa_rect.x(), npa_rect.y());
    let offset = npa_pos - browser_region.pos;
    Ok((offset.x as u32, offset.y as u32))
}
fn nearest_positioned_ancestor(element: Element) -> Result<Element, JsValue> {
    let web_window = web_sys::window().unwrap();
    let mut npa = element;
    loop {
        if let Some(property) = &web_window.get_computed_style(&npa)? {
            match property.get_property_value("position")?.as_str() {
                "" | "static" => {
                    // go to parent
                }
                "absolute" | "relative" | "fixed" | "sticky" => {
                    return Ok(npa);
                }
                _ => {
                    return Err("Unexpected position attribute".into());
                }
            }
        }
        if let Some(parent) = npa.parent_element() {
            npa = parent;
        } else {
            return Ok(npa);
        }
    }
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
