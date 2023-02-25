mod gpu_buffers;
mod gpu_config;
mod gpu_mesh;
mod gpu_texture;
mod gpu_triangle;
mod gpu_vertex;
mod heap_buffer;
mod render_pipeline;
mod shader;

pub use gpu_config::*;
pub use gpu_mesh::*;
pub use gpu_texture::*;
pub use gpu_triangle::*;
pub use gpu_vertex::*;
pub(super) use heap_buffer::*;
pub use render_pipeline::*;
pub use shader::*;

use self::gpu_buffers::GpuBuffers;
use crate::{PaddleResult, Transform};
use js_sys::Uint16Array;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlTexture};

pub(super) struct Gpu {
    gpu_buffers: GpuBuffers,
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

        let gpu_buffers = GpuBuffers::new(gl)?;

        let mut gpu = Self {
            gpu_buffers,
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
            &[("Projection", projection.into())],
        )?;

        Ok(gpu)
    }

    /// Takes the provided mesh and perform one or more draw calls (depending on number of textures & uniform values)
    pub(super) fn perform_draw_calls(
        &mut self,
        buffer: &mut WasmHeapBuffer,
        gl: &WebGlRenderingContext,
        vertices: &[GpuVertex],
        triangles: &[GpuTriangle],
    ) -> PaddleResult<()> {
        buffer.prepare_vertices(vertices, self.active_vertex_descriptor());
        self.upload_vertices(gl, &buffer.vertex_data);

        // Scan through the triangles, adding the indices to the index buffer.
        // Every time the texture or uniform values switch, flush and switch.
        let mut current_texture: Option<&WebGlTexture> = None;
        let mut current_uniforms: &UniformList = &UniformList::default();
        for triangle in triangles.iter() {
            let tex = vertices[triangle.indices[0] as usize].tex();
            let uniform_changed = triangle.uniforms != *current_uniforms;
            let texture_changed = if let Some(img) = tex {
                match current_texture {
                    Some(val) => img != val,
                    None => true,
                }
            } else {
                false
            };

            if texture_changed || uniform_changed {
                self.draw_single_texture(
                    gl,
                    current_texture,
                    // current_uniforms,
                    &buffer.triangle_indices,
                );
                buffer.triangle_indices.clear();

                if let Some(img) = tex {
                    current_texture = Some(img);
                }
                current_uniforms = &triangle.uniforms;
                self.render_pipelines[self.active_render_pipeline]
                    .prepare_uniforms(gl, current_uniforms);
            }
            buffer
                .triangle_indices
                .extend(triangle.indices.iter().map(|n| *n as u16));
        }
        // Flush any remaining triangles
        if !buffer.triangle_indices.is_empty() {
            self.draw_single_texture(gl, current_texture, &buffer.triangle_indices);
            buffer.triangle_indices.clear();
        }
        Ok(())
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
        self.gpu_buffers.ensure_index_buffer_size(gl, indices.len());

        unsafe {
            let array = Uint16Array::view(&indices);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                &array,
                WebGlRenderingContext::STREAM_DRAW,
            );
        }
        if texture.is_some() {
            gl.active_texture(WebGlRenderingContext::TEXTURE0);
            gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, texture);
        }

        // Draw the triangles
        gl.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );
        if texture.is_some() {
            gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, None);
        }
    }
    pub fn active_render_pipeline(&self) -> RenderPipelineHandle {
        self.active_render_pipeline
    }
    pub fn active_vertex_descriptor(&self) -> &VertexDescriptor {
        self.render_pipelines[self.active_render_pipeline].vertex_descriptor()
    }
    /// Set the uniform value for a render pipeline.
    ///
    /// Use this for uniforms that are independent of triangles.
    /// If each geometric shape may use a different value, change the value as part of its Pain.
    pub fn update_uniform(
        &mut self,
        gl: &WebGlRenderingContext,
        rp: RenderPipelineHandle,
        name: &'static str,
        value: &super::gpu::UniformValue,
    ) {
        let stashed_rp = self.active_render_pipeline;
        self.use_render_pipeline(gl, rp);
        self.render_pipelines[rp].prepare_uniform(gl, name, value);
        self.use_render_pipeline(gl, stashed_rp);
    }
}

impl Gpu {
    pub(super) fn custom_drop(&mut self, gl: &WebGlRenderingContext) {
        self.render_pipelines.drop_programs(gl);
        self.gpu_buffers.custom_drop(gl);
        gl.delete_shader(Some(&self.default_fragment_shader));
        gl.delete_shader(Some(&self.default_vertex_shader));
    }
}
