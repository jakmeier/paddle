use super::TexturePosition;
use crate::{quicksilver_compat::graphics::Color, Vector};
use web_sys::WebGlTexture;

#[derive(Clone, Debug)]
/// A vertex for drawing items to the GPU
pub struct GpuVertex {
    /// The position of the vertex in space
    pub pos: Vector,
    /// The image to sample from when drawing the triangle.
    /// When no image is defined, just the colors on the vertices will be used.
    /// If both are defined, the image is blended on top of the colors.
    pub image: Option<TexturePosition>,
    /// The color to blend this vertex with
    pub col: Color,
    /// Z coordinate in range [-1,1]
    pub z: f32,
    /// Additional (custom) attribute values to upload to the GPU. The mesh maintainer has to ensure these values are aligned with the associated `ExtraVertexAttributeDescriptor`.
    pub extra: Option<Vec<f32>>,
}

#[derive(Debug, Clone)]
/// Describes the layout of a vertex as it is uploaded to the GPU.
pub struct VertexDescriptor {
    attributes: Vec<VertexAttributeDescriptor>,
    /// in sizeof f32
    size: u32,
}
#[derive(Debug, Clone)]
/// Describes a single attribute of a vertex
pub struct VertexAttributeDescriptor {
    pub name: &'static str,
    /// in sizeof f32
    pub size: i32,
    pub(crate) source: VertexSource,
}

/// Defines where the vertex data is stored
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VertexSource {
    Pos,
    Texture,
    Color,
    HasTexture,
    ExtraVertexAttribute(usize),
}

impl GpuVertex {
    pub fn new(
        pos: Vector,
        image: Option<TexturePosition>,
        col: Color,
        z: f32,
        extra: Option<Vec<f32>>,
    ) -> Self {
        Self {
            pos,
            image,
            col,
            z,
            extra,
        }
    }
    pub fn has_texture(&self) -> bool {
        self.image.is_some()
    }
    // ST-coordinates (normalized)
    pub fn tex_coordinate(&self) -> Option<Vector> {
        self.image.as_ref().map(|img| img.st)
    }
    pub fn tex(&self) -> Option<&WebGlTexture> {
        self.image.as_ref().map(|img| &img.tex)
    }
}

impl VertexDescriptor {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            size: 0,
        }
    }
    pub fn with_pos(mut self) -> Self {
        self.attributes.push(VertexAttributeDescriptor::new(
            "position",
            3,
            VertexSource::Pos,
        ));
        self.size += 3;
        self
    }
    pub fn with_tex(mut self) -> Self {
        self.attributes.push(VertexAttributeDescriptor::new(
            "tex_coord",
            2,
            VertexSource::Texture,
        ));
        self.attributes.push(VertexAttributeDescriptor::new(
            "uses_texture",
            1,
            VertexSource::HasTexture,
        ));
        self.size += 3;
        self
    }
    pub fn with_col(mut self) -> Self {
        self.attributes.push(VertexAttributeDescriptor::new(
            "color",
            4,
            VertexSource::Color,
        ));
        self.size += 4;
        self
    }

    pub fn with(mut self, attribute: &'static str, size: usize) -> Self {
        self.attributes.push(VertexAttributeDescriptor::new(
            attribute,
            size as i32,
            VertexSource::ExtraVertexAttribute(self.attributes.len()),
        ));
        self.size += size as u32;
        self
    }
    pub fn vertex_size_in_sizeof_f32(&self) -> usize {
        self.size as usize
    }
    pub fn attributes(&self) -> &[VertexAttributeDescriptor] {
        &self.attributes
    }
}
impl Default for VertexDescriptor {
    fn default() -> Self {
        Self {
            attributes: vec![
                VertexAttributeDescriptor::new("position", 3, VertexSource::Pos),
                VertexAttributeDescriptor::new("tex_coord", 2, VertexSource::Texture),
                VertexAttributeDescriptor::new("color", 4, VertexSource::Color),
                VertexAttributeDescriptor::new("uses_texture", 1, VertexSource::HasTexture),
            ],
            size: 10,
        }
    }
}

impl VertexAttributeDescriptor {
    pub fn new(name: &'static str, size: i32, source: VertexSource) -> Self {
        Self { name, size, source }
    }
}
