mod gpu;
mod shader;

use crate::{
    quicksilver_compat::{
        geom::Scalar, Background, Color, Drawable, GpuTriangle, Mesh, Rectangle, Transform, Vector,
        Vertex, View,
    },
    ErrorMessage, PaddleResult,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

use self::gpu::{Gpu, WasmGpuBuffer};

/// TODO: rename to canvas
pub struct Window {
    // resize: ResizeStrategy,
    screen_region: Rectangle,
    view: View,
    mesh: Mesh,
    fullscreen: bool,

    canvas: HtmlCanvasElement,
    gl: WebGlRenderingContext,
    buffer: WasmGpuBuffer,
    gpu: Gpu,
}
impl Window {
    pub fn new(canvas: HtmlCanvasElement) -> PaddleResult<Self> {
        let gl = canvas
            .get_context("webgl")
            .map_err(|_| ErrorMessage::technical("Failed loading WebGL".to_owned()))?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .map_err(|_| ErrorMessage::technical("Failed loading WebGL".to_owned()))?;

        let w = canvas.width();
        let h = canvas.height();
        let screen_region = Rectangle::new_sized((w, h));
        let view = View::new(screen_region);

        let buffer = WasmGpuBuffer::new();
        let gpu = Gpu::new(&gl)?;

        Ok(Window {
            screen_region,
            view,
            // update_rate: settings.update_rate,
            // max_updates: settings.max_updates,
            // draw_rate: settings.draw_rate,
            mesh: Mesh::new(),
            fullscreen: false,
            canvas,
            gl,
            buffer,
            gpu,
        })

        // mouse: Mouse {
        //     pos: Vector::ZERO,
        //     buttons: [ButtonState::NotPressed; 3],
        //     wheel: Vector::ZERO,
        // },
    }

    pub fn from_canvas_id(id: &str) -> PaddleResult<Self> {
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
        Self::new(canvas)
    }

    pub fn html_element(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
    pub fn clone_webgl(&self) -> WebGlRenderingContext {
        self.gl.clone()
    }

    /// Get the screen offset
    pub fn screen_offset(&self) -> Vector {
        self.screen_region.top_left()
    }

    ///Get the screen size
    pub fn screen_size(&self) -> Vector {
        self.screen_region.size()
    }

    ///Get the unprojection matrix according to the View
    pub fn unproject(&self) -> Transform {
        Transform::scale(self.screen_size()) * self.view.normalize
    }

    ///Get the projection matrix according to the View
    pub fn project(&self) -> Transform {
        self.unproject().inverse()
    }

    // ///Handle the available size for the window changing
    // pub(crate) fn adjust_size(&mut self, available: Vector) {
    //     self.screen_region = self.resize.resize(self.screen_region.size(), available);
    //     unsafe {
    //         self.backend().set_viewport(self.screen_region);
    //     }
    // }

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
        todo!()
        // let size = size.into();
        // self.backend().resize(size);
        // self.adjust_size(size);
    }

    /// Flush the current buffered draw calls
    ///
    /// Attributes like z-ordering will be reset: all items drawn after a flush will *always* draw
    /// over all items drawn before a flush.
    ///
    /// Note that calling this can be an expensive operation
    pub fn flush(&mut self) -> PaddleResult<()> {
        // self.mesh.triangles.sort();
        for vertex in self.mesh.vertices.iter_mut() {
            vertex.pos = self.view.opengl * vertex.pos;
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

    // pub fn clear(&mut self, color: Color) {
    //     self.clear_letterbox_color(color, Color::BLACK)
    // }

    // /// Clear the screen to a given color, with a given letterbox color
    // ///
    // /// The blend mode is also automatically reset,
    // /// and any un-flushed draw calls are dropped.
    // pub fn clear_letterbox_color(&mut self, color: Color, letterbox: Color) {
    //     self.mesh.clear();
    //     // self.reset_blend_mode();
    //     self.clear_color(color, letterbox)
    // }

    // fn clear_color(&mut self, color: Color, letterbox: Color) {
    pub fn clear(&mut self, color: Color) {
        self.buffer
            .draw(
                &self.gl,
                &mut self.gpu,
                &[
                    Vertex::new((-1, -1), None, Background::Col(color)),
                    Vertex::new((1, -1), None, Background::Col(color)),
                    Vertex::new((1, 1), None, Background::Col(color)),
                    Vertex::new((-1, 1), None, Background::Col(color)),
                ],
                &[
                    GpuTriangle::new(0, [0, 1, 2], 0.0, Background::Col(color)),
                    GpuTriangle::new(0, [2, 3, 0], 0.0, Background::Col(color)),
                ],
            )
            .expect("Failed to clear");
        self.flush().expect("Failed to flush");
    }
}
