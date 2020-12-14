pub const Z_MIN: i32 = 0;
pub const Z_MAX: i32 = 1000;

use super::gpu::{Gpu, WasmGpuBuffer};
use crate::{
    quicksilver_compat::{
        geom::Scalar, Background, Color, Drawable, Mesh, Rectangle, Transform, Vector,
    },
    ErrorMessage, JsError, NutsCheck, PaddleResult,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub(crate) struct WebGLCanvas {
    /// Resolution used by WebGL
    pixels: Vector,
    mesh: Mesh,
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

        let buffer = WasmGpuBuffer::new();

        // project screen space coordinates with origin at top left and y pointing down,
        // to WebGL's [-1,-1] to [1,1] space with y pointing up
        let projection = Transform::scale((1.0, -1.0))
            * Transform::translate((-1.0, -1.0))
            * Transform::scale(pixels.recip() * 2.0);
        let gpu = Gpu::new(&gl, projection)?;

        let window = WebGLCanvas {
            pixels,
            mesh: Mesh::new(),
            canvas,
            gl,
            buffer,
            gpu,
        };
        Ok(window)
    }

    pub fn html_element(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
    pub fn clone_webgl(&self) -> WebGlRenderingContext {
        self.gl.clone()
    }

    /// Draw a Drawable to the window, which will be finalized on the next flush
    pub fn draw<'a>(&'a mut self, draw: &impl Drawable, bkg: impl Into<Background<'a>>) {
        self.draw_ex(draw, bkg.into(), Transform::IDENTITY, 0.0);
    }

    /// Draw a Drawable to the window with more options provided (draw exhaustive)
    pub fn draw_ex<'a>(
        &'a mut self,
        draw: &impl Drawable,
        bkg: impl Into<Background<'a>>,
        trans: Transform,
        z: impl Scalar,
    ) {
        debug_assert!(z.float() >= Z_MIN as f32);
        debug_assert!(z.float() <= Z_MAX as f32);
        draw.draw(&mut self.mesh, bkg.into(), trans, z.float() / Z_MAX as f32);
    }

    pub(crate) fn mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    /// Resize the area the canvas takes in the browser, (In browser coordinates)
    pub(crate) fn set_size(&mut self, size: impl Into<Vector>) {
        let target_size = size.into();
        self.canvas
            .set_attribute(
                "style",
                &format!("width: {}px; height: {}px", target_size.x, target_size.y),
            )
            .map_err(JsError::from_js_value)
            .map_err(ErrorMessage::from)
            .nuts_check();
    }

    /// Flush the current buffered draw calls
    ///
    /// Attributes like z-ordering will be reset: all items drawn after a flush will *always* draw
    /// over all items drawn before a flush.
    ///
    /// Note that calling this can be an expensive operation
    pub fn flush(&mut self) -> PaddleResult<()> {
        if self.gpu.depth_tests_enabled {
            // If depth tests are enabled, overdrawing can be avoided (for performance) by drawing closer sprites first
            self.mesh.triangles.sort_by(|a, b| b.cmp(a));
            self.gl.clear(WebGlRenderingContext::DEPTH_BUFFER_BIT);
        } else {
            // If depth tests are disabled, overdrawing has to be forced for correctness
            self.mesh.triangles.sort();
        }
        self.buffer.draw(
            &self.gl,
            &mut self.gpu,
            &self.mesh.vertices,
            self.mesh.triangles.as_slice(),
        )?;
        self.mesh.clear();
        Ok(())
    }

    pub fn clear(&mut self, color: Color) {
        self.gl.clear_color(color.r, color.g, color.b, color.a);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
    }
}

impl Drop for WebGLCanvas {
    fn drop(&mut self) {
        self.gpu.custom_drop(&self.gl);
    }
}
