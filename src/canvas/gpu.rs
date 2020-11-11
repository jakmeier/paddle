use js_sys::Float32Array;
use js_sys::Uint16Array;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture};

// TODO: Better way to deal with this?
const VERTEX_SIZE: usize = 9; // the number of floats in a vertex

use crate::{
    graphics::Image,
    quicksilver_compat::{Color, GpuTriangle, Vector, Vertex},
    ErrorMessage, PaddleResult,
};

/// Used to prepare data in GPU readable format for a frame.
/// Once this gives access to JS through a Float32Array view,
/// no memory allocations are allowed until the view is dropped again.
///
/// Using this buffer's interface is safe. The transition to the JS world
/// is contained within the draw method, which incidentally clears this buffer.
pub(crate) struct WasmGpuBuffer {
    vertices: Vec<f32>,
    triangle_indices: Vec<u16>,
}

impl WasmGpuBuffer {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(512),
            triangle_indices: Vec::with_capacity(512),
        }
    }

    fn naive_prepare_vertices(&mut self, vertices: &[Vertex]) {
        // Turn the provided vertex data into stored vertex data
        vertices.iter().for_each(|vertex| {
            self.vertices.push(vertex.pos.x);
            self.vertices.push(vertex.pos.y);
            let tex_pos = vertex.tex_pos.unwrap_or(Vector::ZERO);
            self.vertices.push(tex_pos.x);
            self.vertices.push(tex_pos.y);
            self.vertices.push(vertex.col.r);
            self.vertices.push(vertex.col.g);
            self.vertices.push(vertex.col.b);
            self.vertices.push(vertex.col.a);
            self.vertices
                .push(if vertex.tex_pos.is_some() { 1f32 } else { 0f32 });
        });
    }
    pub(super) fn draw(
        &mut self,
        gl: &WebGlRenderingContext,
        gpu: &mut Gpu,
        vertices: &[Vertex],
        triangles: &[GpuTriangle],
    ) -> PaddleResult<()> {
        self.vertices.clear();
        self.naive_prepare_vertices(vertices);
        gpu.load_vertices(gl, &self.vertices);

        // Scan through the triangles, adding the indices to the index buffer (every time the
        // texture switches, flush and switch the bound texture)
        let mut current_texture: Option<&Image> = None;
        for triangle in triangles.iter() {
            if let Some(ref img) = triangle.image {
                let should_flush = match current_texture {
                    Some(val) => img != val,
                    None => true,
                };
                if should_flush {
                    gpu.draw_single_texture(
                        gl,
                        current_texture.map(|img| img.texture()),
                        &self.triangle_indices,
                    );
                    self.triangle_indices.clear();
                }
                current_texture = Some(img);
            }
            self.triangle_indices
                .extend(triangle.indices.iter().map(|n| *n as u16));
        }
        // Flush any remaining triangles
        gpu.draw_single_texture(
            gl,
            current_texture.map(|img| img.texture()),
            &self.triangle_indices,
        );
        self.triangle_indices.clear();
        Ok(())
    }
}

pub(super) struct Gpu {
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    vertex_buffer_size: usize,
    index_buffer_size: usize,
    program: WebGlProgram,
    fragment_shader: WebGlShader,
    vertex_shader: WebGlShader,
}

impl Gpu {
    pub fn new(gl: &WebGlRenderingContext, coordinate_system: Vector) -> PaddleResult<Self> {
        let vertex_buffer = gl
            .create_buffer()
            .ok_or_else(|| ErrorMessage::technical("failed to create buffer".to_owned()))?;
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        let index_buffer = gl
            .create_buffer()
            .ok_or_else(|| ErrorMessage::technical("failed to create buffer".to_owned()))?;
        gl.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );

        let vertex_shader = super::shader::new_vertex_shader(&gl)?;
        let fragment_shader = super::shader::new_fragment_shader(&gl)?;
        let program = super::shader::link_program(&gl, &vertex_shader, &fragment_shader)?;

        let unform_loc = gl.get_uniform_location(&program, "Outer_resolution");
        gl.uniform2f(
            unform_loc.as_ref(),
            coordinate_system.x,
            coordinate_system.y,
        );

        Ok(Self {
            vertex_buffer,
            index_buffer,
            vertex_buffer_size: 0,
            index_buffer_size: 0,
            vertex_shader,
            fragment_shader,
            program,
        })
    }

    fn load_vertices(&mut self, gl: &WebGlRenderingContext, vertices: &[f32]) {
        let vertex_length = std::mem::size_of::<f32>() * vertices.len();
        // If the GPU can't store all of our data, re-create the GPU buffers so they can
        if vertex_length > self.vertex_buffer_size {
            self.vertex_buffer_size = ceil_pow2(vertex_length);
            // Create the vertex array
            gl.buffer_data_with_i32(
                WebGlRenderingContext::ARRAY_BUFFER,
                self.vertex_buffer_size as i32,
                WebGlRenderingContext::STREAM_DRAW,
            );
            let stride_distance = (VERTEX_SIZE * std::mem::size_of::<f32>()) as i32;
            // Set up the vertex attributes
            let pos_attrib = gl.get_attrib_location(&self.program, "position") as u32;
            gl.enable_vertex_attrib_array(pos_attrib);
            gl.vertex_attrib_pointer_with_i32(
                pos_attrib,
                2,
                WebGlRenderingContext::FLOAT,
                false,
                stride_distance,
                0,
            );
            let tex_attrib = gl.get_attrib_location(&self.program, "tex_coord") as u32;
            gl.enable_vertex_attrib_array(tex_attrib);
            gl.vertex_attrib_pointer_with_i32(
                tex_attrib,
                2,
                WebGlRenderingContext::FLOAT,
                false,
                stride_distance,
                2 * std::mem::size_of::<f32>() as i32,
            );
            let col_attrib = gl.get_attrib_location(&self.program, "color") as u32;
            gl.enable_vertex_attrib_array(col_attrib);
            gl.vertex_attrib_pointer_with_i32(
                col_attrib,
                4,
                WebGlRenderingContext::FLOAT,
                false,
                stride_distance,
                4 * std::mem::size_of::<f32>() as i32,
            );
            let use_texture_attrib = gl.get_attrib_location(&self.program, "uses_texture") as u32;
            gl.enable_vertex_attrib_array(use_texture_attrib);
            gl.vertex_attrib_pointer_with_i32(
                use_texture_attrib,
                1,
                WebGlRenderingContext::FLOAT,
                false,
                stride_distance,
                8 * std::mem::size_of::<f32>() as i32,
            );
            // gl.get_uniform_location(&self.program, "tex");
        }

        // Upload all of the vertex data
        unsafe {
            let array = Float32Array::view(vertices);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                0,
                &array,
            );
        }
    }

    // Assumes that vertices area already uploaded, hence only the indices are needed as parameter
    fn draw_single_texture(
        &mut self,
        gl: &WebGlRenderingContext,
        texture: Option<&WebGlTexture>,
        indices: &[u16],
    ) {
        if indices.is_empty() {
            return;
        }

        // Check if the index buffer is big enough and upload the data
        let index_length = std::mem::size_of::<u32>() * indices.len();
        if index_length > self.index_buffer_size {
            self.index_buffer_size = ceil_pow2(index_length);
            gl.buffer_data_with_i32(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                self.index_buffer_size as i32,
                WebGlRenderingContext::STREAM_DRAW,
            );
        }

        unsafe {
            let array = Uint16Array::view(&indices);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &array,
                WebGlRenderingContext::STREAM_DRAW,
            );
        }
        // Upload the texture to the GPU
        gl.active_texture(WebGlRenderingContext::TEXTURE0);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture);
        if texture.is_some() {
            let texture_mode = WebGlRenderingContext::NEAREST;
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_MIN_FILTER,
                texture_mode as i32,
            );
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_MAG_FILTER,
                texture_mode as i32,
            );
        }
        // TODO: texture location
        gl.uniform1i(None, 0);

        // Draw the triangles
        gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
    pub(crate) fn clear(&mut self, gl: &WebGlRenderingContext, col: Color) {
        gl.clear_color(col.r, col.g, col.b, col.a);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);
    }
}

fn ceil_pow2(x: usize) -> usize {
    let log2_plus1 = num_bits::<usize>() as u32 - (x.saturating_sub(1)).leading_zeros();
    1 << log2_plus1
}

const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

impl Gpu {
    pub(super) fn custom_drop(&mut self, gl: &WebGlRenderingContext) {
        gl.delete_program(Some(&self.program));
        gl.delete_shader(Some(&self.fragment_shader));
        gl.delete_shader(Some(&self.vertex_shader));
        gl.delete_buffer(Some(&self.vertex_buffer));
        gl.delete_buffer(Some(&self.index_buffer));
    }
}

#[test]
fn test_ceil_pow2() {
    let x = 31;
    assert_eq!(32, ceil_pow2(x));

    let x = 32;
    assert_eq!(32, ceil_pow2(x));

    let x = 44;
    assert_eq!(64, ceil_pow2(x));
}
