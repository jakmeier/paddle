//! Managing data regions shared with GPU

use crate::{ErrorMessage, PaddleResult, VertexDescriptor};

use super::Gpu;
use js_sys::Float32Array;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

/// Manages an index buffer and one buffer for each vertex kind used (on the GPU side)
pub(super) struct GpuBuffers {
    index_buffer: IndexBuffer,
    vertex_buffers: Vec<VertexBuffer>,
}

/// Indices for triangle definition. Simple format is used where each triangle has is own three indices in the buffer.
struct IndexBuffer {
    buffer: WebGlBuffer,
    size: usize,
}
/// Stores the vertices referenced by triangle indices.
struct VertexBuffer {
    buffer: WebGlBuffer,
    size: usize,
    v_desc: VertexDescriptor,
}

impl Gpu {
    /// Copy heap-buffered data over to GPU buffers
    pub(super) fn upload_vertices(&mut self, gl: &WebGlRenderingContext, vertices: &[f32]) {
        // Bind the correct vertex buffer
        let render_config = self.active_render_pipeline;
        let vertex_buffer = &mut self.gpu_buffers.vertex_buffers[render_config.num()];
        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&vertex_buffer.buffer),
        );
        // If the GPU can't store all of our data, re-create the GPU buffers so they can
        let program = &self.render_pipelines[self.active_render_pipeline].program();
        let vertex_length = std::mem::size_of::<f32>() * vertices.len();
        vertex_buffer.ensure_size(gl, vertex_length);
        // TODO: Check if this hurts performance. I don' really understand why this needs to be called every time, I thought only after resizing but it fails when using multiple render pipelines.
        vertex_buffer.prepare_buffer_layout(gl, program);

        // Upload all of the vertex data
        unsafe {
            let array = Float32Array::view(vertices);
            gl.buffer_sub_data_with_i32_and_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                0,
                &array,
            );
        }
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);
    }
}

impl GpuBuffers {
    pub fn new(gl: &WebGlRenderingContext) -> PaddleResult<Self> {
        let vertex_buffers = vec![];

        let index_buffer = IndexBuffer::new(gl)?;
        Ok(Self {
            vertex_buffers,
            index_buffer,
        })
    }
    pub(super) fn add_vertex_buffer(
        &mut self,
        gl: &WebGlRenderingContext,
        v_desc: VertexDescriptor,
    ) {
        self.vertex_buffers
            .push(VertexBuffer::new(gl, v_desc).expect("Failed creating vertex buffer"))
    }
    pub(super) fn ensure_index_buffer_size(&mut self, gl: &WebGlRenderingContext, size: usize) {
        self.index_buffer.ensure_size(gl, size);
    }
    pub(super) fn custom_drop(&mut self, gl: &WebGlRenderingContext) {
        gl.delete_buffer(Some(&self.index_buffer.buffer));
        for buffer in &self.vertex_buffers {
            gl.delete_buffer(Some(&buffer.buffer));
        }
    }
}

impl IndexBuffer {
    pub fn new(gl: &WebGlRenderingContext) -> PaddleResult<Self> {
        let buffer = gl
            .create_buffer()
            .ok_or_else(|| ErrorMessage::technical("failed to create buffer".to_owned()))?;
        gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
        Ok(Self { buffer, size: 0 })
    }
    fn ensure_size(&mut self, gl: &WebGlRenderingContext, size: usize) {
        let index_length = std::mem::size_of::<u32>() * size;
        if index_length > self.size {
            self.size = index_length.next_power_of_two();
            gl.buffer_data_with_i32(
                WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
                self.size as i32,
                WebGlRenderingContext::STREAM_DRAW,
            );
        }
    }
}

impl VertexBuffer {
    pub fn new(gl: &WebGlRenderingContext, v_desc: VertexDescriptor) -> PaddleResult<Self> {
        let buffer = gl
            .create_buffer()
            .ok_or_else(|| ErrorMessage::technical("failed to create buffer".to_owned()))?;
        Ok(Self {
            buffer,
            size: 0,
            v_desc,
        })
    }
    fn ensure_size(&mut self, gl: &WebGlRenderingContext, size: usize) {
        if size > self.size {
            self.size = size.next_power_of_two();
            gl.buffer_data_with_i32(
                WebGlRenderingContext::ARRAY_BUFFER,
                self.size as i32,
                WebGlRenderingContext::STREAM_DRAW,
            );
        }
    }
    fn prepare_buffer_layout(&self, gl: &WebGlRenderingContext, program: &WebGlProgram) {
        let vertex_size = self.v_desc.vertex_size_in_sizeof_f32();
        let stride_distance = (vertex_size * std::mem::size_of::<f32>()) as i32;

        let mut offset = 0;
        for attribute in self.v_desc.attributes() {
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
}
