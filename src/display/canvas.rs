pub const Z_MIN: i16 = 0;
pub const Z_MAX: i16 = 32_767i16;

pub(crate) const ABSTRACT_SPACE: Rectangle = Rectangle {
    pos: Vector { x: -1.0, y: -1.0 },
    size: Vector { x: 2.0, y: 2.0 },
};

use super::gpu::{
    new_fragment_shader, new_vertex_shader, Gpu, GpuConfig, GpuMesh, RenderPipelineHandle,
    UniformValue, VertexDescriptor, WasmHeapBuffer,
};
use crate::{
    quicksilver_compat::Color, ErrorMessage, JsError, NutsCheck, PaddleResult, Paint, Rectangle,
    Render, Transform, Vector,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub(crate) struct WebGLCanvas {
    /// Resolution used by WebGL
    pixels: Vector,
    mesh: GpuMesh,
    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
    buffer: WasmHeapBuffer,
    gpu: Gpu,
}
impl WebGLCanvas {
    /// Create a new webgl area in the given canvas.
    ///
    /// The pixels argument define how many webgl pixels should be rendered and has nothing to do with browser pixels.
    /// Use `set_size()` or `fit_to_screen()` to change the size of the screen area taken by this element.
    pub fn new(
        canvas: HtmlCanvasElement,
        pixels: impl Into<Vector>,
        gpu_config: &GpuConfig,
    ) -> PaddleResult<Self> {
        let pixels = pixels.into();
        canvas.set_width(pixels.x as u32);
        canvas.set_height(pixels.y as u32);
        canvas
            .set_attribute(
                "style",
                &format!("width: {}px; height: {}px", pixels.x, pixels.y),
            )
            .map_err(|_| ErrorMessage::technical("Failed setting canvas style".to_owned()))?;

        let gl = canvas
            .get_context("webgl")
            .map_err(|_| ErrorMessage::technical("Failed loading WebGL".to_owned()))?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| ErrorMessage::technical("Failed loading WebGL".to_owned()))?;

        let buffer = WasmHeapBuffer::new();

        // project screen space coordinates with origin at top left and y pointing down,
        // to WebGL's [-1,-1] to [1,1] space with y pointing up
        let projection = Transform::scale((1.0, -1.0))
            * Transform::translate((-1.0, -1.0))
            * Transform::scale(pixels.recip() * 2.0);
        let gpu = Gpu::new(&gl, projection, gpu_config)?;

        let window = WebGLCanvas {
            pixels,
            mesh: GpuMesh::new(),
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
    /// How many pixels are rendered in the Canvas
    pub fn resolution(&self) -> Vector {
        self.pixels
    }

    /// Render object to the display buffer, to be forwarded to the GPU on the next flush
    pub fn render(&mut self, draw: &impl Render, trans: &Transform, paint: &impl Paint, z: i16) {
        debug_assert!(z >= Z_MIN);
        debug_assert!(z <= Z_MAX);
        self.ensure_render_pipeline(paint.paint_render_pipeline())
            .expect("Failed to set render pipeline");
        draw.render(&mut self.mesh, trans, paint, z);
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
        self.gpu.perform_draw_calls(
            &mut self.buffer,
            &self.gl,
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
    /// If this RP is not already active, buffers will be flushed and RP is set
    pub fn ensure_render_pipeline(&mut self, rp: RenderPipelineHandle) -> PaddleResult<()> {
        if rp != self.gpu.active_render_pipeline() {
            self.flush()?;
            self.gpu.use_render_pipeline(&self.gl, rp);
        }
        Ok(())
    }

    pub fn new_render_pipeline(
        &mut self,
        vertex_shader_text: &str,
        fragment_shader_text: &str,
        vertex_descriptor: VertexDescriptor,
        uniform_values: &[(&'static str, UniformValue)],
    ) -> PaddleResult<RenderPipelineHandle> {
        let vertex_shader = new_vertex_shader(&self.gl, vertex_shader_text)?;
        let fragment_shader = new_fragment_shader(&self.gl, fragment_shader_text)?;

        self.gpu.new_render_pipeline(
            &self.gl,
            vertex_shader,
            fragment_shader,
            vertex_descriptor,
            uniform_values,
        )
    }
    pub fn update_uniform(
        &mut self,
        rp: RenderPipelineHandle,
        name: &'static str,
        value: &super::gpu::UniformValue,
    ) {
        self.gpu.update_uniform(&self.gl, rp, name, value)
    }
}

impl Drop for WebGLCanvas {
    fn drop(&mut self) {
        self.gpu.custom_drop(&self.gl);
    }
}
