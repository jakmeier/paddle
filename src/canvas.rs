mod gpu;
mod shader;

use crate::{
    quicksilver_compat::{
        geom::Scalar, Background, Color, Drawable, Mesh, Rectangle, Transform, Vector, View,
    },
    ErrorMessage, JsError, PaddleResult,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use self::gpu::{Gpu, WasmGpuBuffer};

pub struct WebGLCanvas {
    /// Position relative to browser page
    browser_region: Rectangle,
    /// Resolution used by WebGL
    pixels: Vector,
    view: View,
    mesh: Mesh,
    fullscreen: bool,

    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
    buffer: WasmGpuBuffer,
    gpu: Gpu,
}
impl WebGLCanvas {
    /// Create a new webgl area in the given canvas.
    ///
    /// The pixels argument define how many webgl pixels should be rendered and has nothing to do with browser pixels.
    /// Use `set_size()` or `fit_to_screen()` to change the size of the screen area taken by this element.
    pub fn new(canvas: HtmlCanvasElement, pixels: impl Into<Vector>) -> PaddleResult<Self> {
        let pixels = pixels.into();
        canvas.set_width(pixels.x as u32);
        canvas.set_height(pixels.y as u32);

        let gl = canvas
            .get_context("webgl")
            .map_err(|_| ErrorMessage::technical("Failed loading WebGL".to_owned()))?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| ErrorMessage::technical("Failed loading WebGL".to_owned()))?;

        let web_window = web_sys::window().unwrap();
        let dom_rect = canvas.get_bounding_client_rect();

        let page_x_offset = web_window.page_x_offset().map_err(JsError::from_js_value)?;
        let page_y_offset = web_window.page_y_offset().map_err(JsError::from_js_value)?;
        let x = dom_rect.x() + page_x_offset;
        let y = dom_rect.y() + page_y_offset;
        let w = dom_rect.width() + page_x_offset;
        let h = dom_rect.height() + page_y_offset;

        let browser_region = Rectangle::new((x as f32, y as f32), (w as f32, h as f32));
        let view = View::new(Rectangle::new_sized(pixels));

        let buffer = WasmGpuBuffer::new();
        let gpu = Gpu::new(&gl, pixels)?;

        let window = WebGLCanvas {
            browser_region,
            view,
            pixels,
            mesh: Mesh::new(),
            fullscreen: false,
            canvas,
            gl,
            buffer,
            gpu,
        };
        Ok(window)
    }

    pub fn from_canvas_id(id: &str, w: i32, h: i32) -> PaddleResult<Self> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(id)
            .ok_or_else(|| ErrorMessage::technical(format!("No canvas with id {}", id)))?;
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().map_err(|e| {
            ErrorMessage::technical(format!(
                "Not a canvas. Err: {}",
                e.to_string().as_string().unwrap()
            ))
        })?;
        Self::new(canvas, (w, h))
    }

    pub fn html_element(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
    pub fn clone_webgl(&self) -> WebGlRenderingContext {
        self.gl.clone()
    }

    /// Position relative to browser page and size in browser pixels
    pub fn browser_region(&self) -> Rectangle {
        self.browser_region
    }

    ///Get the unprojection matrix according to the View
    pub fn unproject(&self) -> Transform {
        Transform::scale(self.browser_region().size()) * self.view.normalize
    }

    ///Get the projection matrix according to the View
    pub fn project(&self) -> Transform {
        self.unproject().inverse()
    }

    ///Get the view from the window
    pub fn view(&self) -> View {
        self.view
    }

    ///Set the view the window uses
    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    /// Draw a Drawable to the window, which will be finalized on the next flush
    pub fn draw<'a>(&'a mut self, draw: &impl Drawable, bkg: impl Into<Background<'a>>) {
        self.draw_ex(draw, bkg.into(), Transform::IDENTITY, 0);
    }

    /// Draw a Drawable to the window with more options provided (draw exhaustive)
    pub fn draw_ex<'a>(
        &'a mut self,
        draw: &impl Drawable,
        bkg: impl Into<Background<'a>>,
        trans: Transform,
        z: impl Scalar,
    ) {
        draw.draw(&mut self.mesh, bkg.into(), trans, z);
    }

    /// The mesh the window uses to draw
    pub fn mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    /// Get if the application is currently fullscreen
    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Resize the window to the given size
    pub fn set_size(&mut self, size: impl Into<Vector>) {
        self.browser_region.size = size.into();
        self.canvas.set_attribute(
            "style",
            &format!(
                "width: {}px; height: {}px",
                self.browser_region.size.x, self.browser_region.size.y
            ),
        );
    }

    /// Flush the current buffered draw calls
    ///
    /// Attributes like z-ordering will be reset: all items drawn after a flush will *always* draw
    /// over all items drawn before a flush.
    ///
    /// Note that calling this can be an expensive operation
    pub fn flush(&mut self) -> PaddleResult<()> {
        self.buffer.draw(
            &self.gl,
            &mut self.gpu,
            &self.mesh.vertices,
            self.mesh.triangles.as_slice(),
        )?;
        self.mesh.clear();
        Ok(())
    }

    // 16:9 ratio
    pub fn fit_to_screen(&mut self, margin: f64) -> PaddleResult<()> {
        let web_window = web_sys::window().unwrap();
        let page_x_offset = web_window.page_x_offset().map_err(JsError::from_js_value)?;
        let page_y_offset = web_window.page_y_offset().map_err(JsError::from_js_value)?;
        let dom_rect = self.canvas.get_bounding_client_rect();
        let x = dom_rect.x() + page_x_offset;
        let y = dom_rect.y() + page_y_offset;

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

        let (w, h) = scale_16_to_9(w - x - margin, h - y - margin);

        self.set_size((w as f32, h as f32));
        Ok(())
    }

    pub fn clear(&mut self, color: Color) {
        self.gl.clear_color(color.r, color.g, color.b, color.a);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
    }
}

fn scale_16_to_9(w: f64, h: f64) -> (f64, f64) {
    if w * 9.0 > h * 16.0 {
        (h * 16.0 / 9.0, h)
    } else {
        (w, w * 9.0 / 16.0)
    }
}

impl Drop for WebGLCanvas {
    fn drop(&mut self) {
        self.gpu.custom_drop(&self.gl);
    }
}
