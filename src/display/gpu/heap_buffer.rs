use super::VertexSource;
use crate::{GpuVertex, VertexDescriptor};

/// Manages memory regions in the WASM heap that is used
/// to prepare data in GPU readable format for a frame.
/// Once this gives access to JS through a Float32Array view,
/// no memory allocations are allowed until the view is dropped again.
///
/// Using this buffer's interface is safe. The transition to the JS world
/// is contained within the draw method, which incidentally clears this buffer.
pub(crate) struct WasmHeapBuffer {
    pub(super) vertex_data: Vec<f32>,
    pub(super) triangle_indices: Vec<u16>,
}

impl WasmHeapBuffer {
    pub fn new() -> Self {
        Self {
            vertex_data: Vec::with_capacity(512),
            triangle_indices: Vec::with_capacity(512),
        }
    }

    /// Prepare vertex attributes in a heap-backed buffer to allow memory copy into GPU buffers
    pub(super) fn prepare_vertices(&mut self, vertices: &[GpuVertex], v_desc: &VertexDescriptor) {
        self.vertex_data.clear();
        vertices.iter().for_each(|vertex| {
            for attr in v_desc.attributes() {
                match attr.source {
                    VertexSource::Pos => {
                        // attribute vec3 position;
                        self.vertex_data.push(vertex.pos.x);
                        self.vertex_data.push(vertex.pos.y);
                        debug_assert!(vertex.z <= 1.0);
                        debug_assert!(vertex.z >= -1.0);
                        self.vertex_data.push(vertex.z);
                    }
                    VertexSource::Texture => {
                        // attribute vec2 tex_coord;
                        let tex_pos = vertex.tex_coordinate();
                        self.vertex_data.push(tex_pos.x);
                        self.vertex_data.push(tex_pos.y);
                    }
                    VertexSource::Color => {
                        // attribute vec4 color;
                        self.vertex_data.push(vertex.col.r);
                        self.vertex_data.push(vertex.col.g);
                        self.vertex_data.push(vertex.col.b);
                        self.vertex_data.push(vertex.col.a);
                    }
                    VertexSource::HasTexture => {
                        // attribute lowp float uses_texture;
                        self.vertex_data
                            .push(if vertex.has_texture() { 1.0 } else { 0.0 });
                    }
                    VertexSource::ExtraVertexAttribute(i) => {
                        self.vertex_data.push(vertex.extra.as_ref().unwrap()[i]);
                    }
                }
            }
        });
    }
}
