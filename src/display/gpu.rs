mod gpu_config;
mod gpu_mesh;
mod gpu_texture;
mod gpu_triangle;
mod gpu_vertex;
mod render_pipeline;
mod shader;

pub use gpu_config::*;
pub use gpu_mesh::*;
pub use gpu_texture::*;
pub use gpu_triangle::*;
pub use gpu_vertex::*;
pub use render_pipeline::*;
pub use shader::*;

use crate::Transform;
use js_sys::Float32Array;
use js_sys::Uint16Array;
use web_sys::{WebGlBuffer, WebGlRenderingContext, WebGlShader, WebGlTexture};

use crate::{ErrorMessage, PaddleResult, Vector};

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

    /// Copy heap-buffered data over to GPU buffers
    fn prepare_vertices(&mut self, vertices: &[GpuVertex], v_desc: &VertexDescriptor) {
        vertices.iter().for_each(|vertex| {
            for attr in v_desc.attributes() {
                match attr.source {
                    VertexSource::Pos => {
                        // attribute vec3 position;
                        self.vertices.push(vertex.pos.x);
                        self.vertices.push(vertex.pos.y);
                        debug_assert!(vertex.z <= 1.0);
                        debug_assert!(vertex.z >= -1.0);
                        self.vertices.push(vertex.z);
                    }
                    VertexSource::Texture => {
                        // attribute vec2 tex_coord;
                        let tex_pos = vertex.tex_coordinate().unwrap_or(Vector::ZERO);
                        self.vertices.push(tex_pos.x);
                        self.vertices.push(tex_pos.y);
                    }
                    VertexSource::Color => {
                        // attribute vec4 color;
                        self.vertices.push(vertex.col.r);
                        self.vertices.push(vertex.col.g);
                        self.vertices.push(vertex.col.b);
                        self.vertices.push(vertex.col.a);
                    }
                    VertexSource::HasTexture => {
                        // attribute lowp float uses_texture;
                        self.vertices
                            .push(if vertex.has_texture() { 1.0 } else { 0.0 });
                    }
                    VertexSource::ExtraVertexAttribute(i) => {
                        self.vertices.push(vertex.extra.as_ref().unwrap()[i]);
                    }
                }
            }
        });
    }
    pub(super) fn draw(
        &mut self,
        gl: &WebGlRenderingContext,
        gpu: &mut Gpu,
        vertices: &[GpuVertex],
        triangles: &[GpuTriangle],
    ) -> PaddleResult<()> {
        self.vertices.clear();
        self.prepare_vertices(vertices, gpu.active_vertex_descriptor());
        gpu.load_vertices(gl, &self.vertices);

        // Scan through the triangles, adding the indices to the index buffer (every time the
        // texture switches, flush and switch the bound texture)
        let mut current_texture: Option<&WebGlTexture> = None;
        for triangle in triangles.iter() {
            if let Some(img) = vertices[triangle.indices[0] as usize].tex() {
                let should_flush = match current_texture {
                    Some(val) => img != val,
                    None => true,
                };
                if should_flush {
                    gpu.draw_single_texture(gl, current_texture, &self.triangle_indices);
                    self.triangle_indices.clear();
                }
                current_texture = Some(img);
            }
            self.triangle_indices
                .extend(triangle.indices.iter().map(|n| *n as u16));
        }
        // Flush any remaining triangles
        if !self.triangle_indices.is_empty() {
            gpu.draw_single_texture(gl, current_texture, &self.triangle_indices);
            self.triangle_indices.clear();
        }
        Ok(())
    }
}

pub(super) struct Gpu {
    vertex_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    vertex_buffer_size: usize,
    index_buffer_size: usize,
    default_fragment_shader: WebGlShader,
    default_vertex_shader: WebGlShader,
    active_render_pipeline: RenderPipelineHandle,
    render_pipelines: RenderPipelineContainer,
    // texture_location: Option<WebGlUniformLocation>,
    pub(crate) depth_tests_enabled: bool,
}

impl Gpu {
    pub fn new(
        gl: &WebGlRenderingContext,
        projection: Transform,
        config: &GpuConfig,
    ) -> PaddleResult<Self> {
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

        gl.blend_func_separate(
            WebGlRenderingContext::SRC_ALPHA,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
            WebGlRenderingContext::ONE,
            WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
        );
        gl.enable(WebGlRenderingContext::BLEND);

        let mut depth_tests_enabled = false;
        if config.depth_test {
            // If we can, we want to use the depth buffer for z ordering
            gl.enable(WebGlRenderingContext::DEPTH_TEST);
            depth_tests_enabled = gl.is_enabled(WebGlRenderingContext::DEPTH_TEST);
            if depth_tests_enabled {
                gl.clear_depth(0.0);
                gl.depth_func(WebGlRenderingContext::GEQUAL);
            }
        }

        let default_vertex_shader = new_vertex_shader(&gl, DEFAULT_VERTEX_SHADER)?;
        let default_fragment_shader = new_fragment_shader(&gl, DEFAULT_FRAGMENT_SHADER)?;

        let render_pipelines = RenderPipelineContainer::new();

        let mut gpu = Self {
            vertex_buffer,
            index_buffer,
            vertex_buffer_size: 0,
            index_buffer_size: 0,
            default_vertex_shader,
            default_fragment_shader,
            render_pipelines,
            depth_tests_enabled,
            active_render_pipeline: Default::default(),
        };

        // Register default pipeline (Necessary to make `active_render_pipeline: Default::default()` valid)
        gpu.new_render_pipeline(
            gl,
            gpu.default_vertex_shader.clone(),
            gpu.default_fragment_shader.clone(),
            VertexDescriptor::default(),
            &[("Projection", UniformValue::Matrix3fv(projection.as_slice()))],
        )?;

        Ok(gpu)
    }

    fn load_vertices(&mut self, gl: &WebGlRenderingContext, vertices: &[f32]) {
        let vertex_length = std::mem::size_of::<f32>() * vertices.len();
        // If the GPU can't store all of our data, re-create the GPU buffers so they can
        if vertex_length > self.vertex_buffer_size {
            self.vertex_buffer_size = ceil_pow2(vertex_length);
            self.recreate_vertex_buffer(gl);
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

    fn recreate_vertex_buffer(&mut self, gl: &WebGlRenderingContext) {
        let v_desc = &self.render_pipelines[self.active_render_pipeline].vertex_descriptor();
        gl.buffer_data_with_i32(
            WebGlRenderingContext::ARRAY_BUFFER,
            self.vertex_buffer_size as i32,
            WebGlRenderingContext::STREAM_DRAW,
        );
        let vertex_size = v_desc.vertex_size_in_sizeof_f32();
        let stride_distance = (vertex_size * std::mem::size_of::<f32>()) as i32;
        let program = self.active_program();

        let mut offset = 0;
        for attribute in v_desc.attributes() {
            // Set up the vertex attributes
            let loc = gl.get_attrib_location(program, attribute.name) as u32;
            gl.enable_vertex_attrib_array(loc);
            gl.vertex_attrib_pointer_with_i32(
                loc,
                attribute.size,
                WebGlRenderingContext::FLOAT,
                false,
                stride_distance,
                offset * std::mem::size_of::<f32>() as i32,
            );
            offset += attribute.size;
        }

        debug_assert!(offset as usize == vertex_size);
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
        gl.active_texture(WebGlRenderingContext::TEXTURE0);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture);

        // Draw the triangles
        gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
    }
    pub fn active_render_pipeline(&self) -> RenderPipelineHandle {
        self.active_render_pipeline
    }
    pub fn active_vertex_descriptor(&self) -> &VertexDescriptor {
        self.render_pipelines[self.active_render_pipeline].vertex_descriptor()
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
        self.render_pipelines.drop_programs(gl);
        gl.delete_shader(Some(&self.default_fragment_shader));
        gl.delete_shader(Some(&self.default_vertex_shader));
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
