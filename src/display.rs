//! Parent module around code that puts stuff on the screen.
//!
//! This covers an HTML canvas with WebGL (implemented in Paddle itself), as well as HTML display through the crate `div`.
//! While `Display` covers the full screen, `DisplayArea` are views into parts of tje display, used for light-weight window management.
//!
//! In Paddle, drawing an object to the WebGL canvas consists of two separate phases on the CPU, tesselation + rendering.
//! The display accepts pre-tessellated and raw objects, using either `draw_mesh()` or `draw()` (on DisplayArea).

mod canvas;
mod display_area;
mod display_paint;
mod display_tessellate;
mod fit_strategy;
mod gpu;
mod render;
mod text;

pub use canvas::*;
pub use display_area::*;
pub use display_paint::DisplayPaint;
pub use display_tessellate::DisplayTessellate;
pub use fit_strategy::FitStrategy;
pub use gpu::{
    CustomShader, GpuConfig, GpuMesh, GpuTriangle, GpuVertex, RenderPipelineHandle,
    UniformDescriptor, UniformList, UniformValue, VertexDescriptor,
};
pub use render::*;
pub use text::*;

use crate::*;
use crate::{graphics::AbstractMesh, Vector};
use crate::{graphics::ImageLoader, graphics::TextureConfig, quicksilver_compat::Color};
use div::DivHandle;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlCanvasElement};

/// A stateful object to manage the *full* display area for your game inside the browser.
///
/// The `Display` object is responsible for putting stuff to see in front of the user.
/// In order to do that effectively, it manages some state about the browser. (WebGL, DOM nodes, images)
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
    div: DivHandle,
    /// Buffer for on-the-fly tessellation
    tessellation_buffer: AbstractMesh,
    /// Sprites for images and animations
    asset_library: AssetLibrary,
}

pub struct DisplayConfig {
    pub canvas: CanvasConfig,
    pub pixels: Vector,
    pub texture_config: TextureConfig,
    pub gpu_config: GpuConfig,
    pub update_delay_ms: i32,
    pub background: Option<Color>,
    pub capture_touch: bool,
}
impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            canvas: CanvasConfig::HtmlId("paddle-canvas"),
            pixels: Vector::new(1280, 720),
            update_delay_ms: 8,
            texture_config: Default::default(),
            gpu_config: Default::default(),
            background: None,
            capture_touch: true,
        }
    }
}

pub enum CanvasConfig {
    HtmlId(&'static str),
    HtmlElement(HtmlCanvasElement),
}

use crate::{NutsCheck, PaddleResult, Rectangle, Transform};

impl Display {
    pub(super) fn new(config: DisplayConfig) -> PaddleResult<Self> {
        let canvas = match config.canvas {
            CanvasConfig::HtmlElement(el) => el,
            CanvasConfig::HtmlId(id) => canvas_by_id(id)?,
        };
        let parent_element = canvas.parent_element().expect("Canvas has no parent");
        if config.capture_touch {
            let parent_html = parent_element
                .clone()
                .dyn_into::<web_sys::HtmlElement>()
                .expect("Canvas parent is not an HTML element");
            parent_html
                .style()
                .set_property("touch-action", "none")
                .expect("Setting CSS failed");
        }

        // For now the only option is game_coordinates = pixels
        let game_coordinates = config.pixels;

        let canvas = WebGLCanvas::new(canvas, config.pixels, &config.gpu_config)?;
        // Browser region is relative to window and needs to be known to handle input
        let browser_region = find_browser_region(canvas.html_element())?;

        // Initialize with game coordinates, which allows using them again for later calls
        let size = (game_coordinates.x as u32, game_coordinates.y as u32);
        div::init_ex_with_element(parent_element, (0, 0), Some(size))
            .expect("Div initialization failed");

        div::resize(
            browser_region.width() as u32,
            browser_region.height() as u32,
        )
        .expect("Div initialization failed");

        // For binding textures as they arrive
        ImageLoader::register(canvas.clone_webgl(), config.texture_config);

        let background_color = config.background;

        let div = div::new_styled::<_, _, &'static str, _, _>(
            0,
            0,
            size.0,
            size.1,
            "",
            &[],
            &[("pointer-events", "None")],
        )?;
        div.set_css("z-index", &(-1).to_string())?;

        Ok(Self {
            asset_library: AssetLibrary::default(),
            canvas,
            browser_region,
            game_coordinates,
            background_color,
            div,
            tessellation_buffer: AbstractMesh::new(),
        })
    }
    pub(crate) fn canvas_mut(&mut self) -> &mut WebGLCanvas {
        &mut self.canvas
    }

    /// Position relative to browser page and size in browser pixels
    pub fn browser_region(&self) -> Rectangle {
        self.browser_region
    }
    /// How many pixels are rendered in the Canvas
    pub fn resolution(&self) -> Vector {
        self.canvas.resolution()
    }
    /// Transformation to go from display coordinates to WebGL coordinates. Useful for custom shaders.
    pub fn webgl_transform(&self) -> Transform {
        Transform::scale((1.0, -1.0))
            * Transform::translate((-1.0, -1.0))
            * Transform::scale(self.resolution().recip() * 2.0)
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
    /// Transforms an area from coordinates used inside the game (aka world coordinates) to browser coordinates (as used by e.g. CSS pixels)
    pub fn game_to_browser_area(&self, mut rect: Rectangle) -> Rectangle {
        rect.pos += self.browser_region.pos;
        rect.size = rect.size.times(
            self.browser_region
                .size
                .times(self.game_coordinates.recip()),
        );
        rect
    }

    /// Gives result for x axis (assuming y is the same)
    pub fn browser_to_game_pixel_ratio(&self) -> f32 {
        self.browser_region.width() / self.game_coordinates.x
    }

    /// Scale the display to make it fully visible keeping the ratio true.
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

        let (w, h) = scale_to_ratio(
            w - self.browser_region.x() as f64 - margin,
            h - self.browser_region.y() as f64 - margin,
            self.browser_region.width() as f64,
            self.browser_region.height() as f64,
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
        let web_window = web_sys::window().unwrap();

        let (x, y) = self.div_offset()?;
        let (scroll_x, scroll_y) = (
            web_window.scroll_x().unwrap() as i32,
            web_window.scroll_y().unwrap() as i32,
        );
        div::reposition(x as i32 - scroll_x, y as i32 - scroll_y)?;
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

    /// Draw on the display with exhaustive options
    pub fn draw_ex(
        &mut self,
        position: Option<(&Rectangle, FitStrategy)>,
        shape: &impl DisplayTessellate,
        paint: &impl DisplayPaint,
        trans: &Transform,
        z: i16,
    ) {
        // TODO: Keep tesselation for frame and apply transformation once per frame (potentially on GPU)
        self.tessellation_buffer.clear();
        shape.tessellate(&self.asset_library, &mut self.tessellation_buffer);
        let base_area = shape
            .bounding_box(&self.asset_library)
            .nuts_check()
            .unwrap_or(ABSTRACT_SPACE);
        let final_position = match position {
            Some((pos, fit_strat)) => base_area.fit_into_ex(pos, fit_strat, true),
            None => base_area,
        };
        let trans = *trans * ABSTRACT_SPACE.project(&final_position);
        self.canvas.render(
            &self.tessellation_buffer,
            &trans,
            &(paint, &self.asset_library),
            z,
        );
    }
    // Insert triangles to buffer with a transform and z value
    pub fn draw_mesh_ex(
        &mut self,
        mesh: &AbstractMesh,
        paint: &impl DisplayPaint,
        area: Rectangle,
        t: &Transform,
        z: i16,
    ) {
        let trans = *t * ABSTRACT_SPACE.project(&area);
        self.canvas
            .render(mesh, &trans, &(paint, &self.asset_library), z);
    }
    // TODO: Find a better way to expose this
    pub fn new_render_pipeline(
        &mut self,
        vertex_shader_text: &str,
        fragment_shader_text: &str,
        vertex_descriptor: VertexDescriptor,
        uniform_values: &[(&'static str, UniformValue)],
    ) -> crate::PaddleResult<crate::RenderPipelineHandle> {
        self.canvas.new_render_pipeline(
            vertex_shader_text,
            fragment_shader_text,
            vertex_descriptor,
            uniform_values,
        )
    }
    pub fn update_uniform(
        &mut self,
        rp: RenderPipelineHandle,
        name: &'static str,
        value: &UniformValue,
    ) {
        self.canvas.update_uniform(rp, name, value)
    }
    pub(super) fn asset_library(&mut self) -> &mut AssetLibrary {
        &mut self.asset_library
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
    let offset = browser_region.pos - npa_pos;
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

fn scale_to_ratio(w: f64, h: f64, w_ratio: f64, h_ratio: f64) -> (f64, f64) {
    if w * h_ratio > h * w_ratio {
        (h * w_ratio / h_ratio, h)
    } else {
        (w, w * h_ratio / w_ratio)
    }
}
