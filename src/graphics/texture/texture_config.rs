use web_sys::WebGlRenderingContext;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TextureConfig {
    pub(crate) magnification_filter: MagnificationFilter,
    pub(crate) minification_filter: MinificationFilter,
    pub(crate) mipmap_level: MipmapLevel,
    pub(crate) color_format: ColorFormat,
}

impl TextureConfig {
    pub fn with_pixellated_mangification(mut self) -> Self {
        self.magnification_filter = MagnificationFilter::Nearest;
        self
    }
    pub fn with_blurred_mangification(mut self) -> Self {
        self.magnification_filter = MagnificationFilter::Linear;
        self
    }
    pub fn without_filter(mut self) -> Self {
        self.mipmap_level = MipmapLevel::Off;
        self.minification_filter = MinificationFilter::Nearest;
        self
    }
    pub fn with_unfiltered_mipmap(mut self) -> Self {
        self.mipmap_level = MipmapLevel::Single;
        self.minification_filter = MinificationFilter::Nearest;
        self
    }
    pub fn with_bilinear_filtering(mut self) -> Self {
        self.mipmap_level = MipmapLevel::Single;
        self.minification_filter = MinificationFilter::Linear;
        self
    }
    pub fn with_trilinear_filtering(mut self) -> Self {
        self.mipmap_level = MipmapLevel::Double;
        self.minification_filter = MinificationFilter::Linear;
        self
    }
    pub fn with_rgba(mut self) -> Self {
        self.color_format = ColorFormat::RGBA;
        self
    }
    pub fn with_rgb(mut self) -> Self {
        self.color_format = ColorFormat::RGB;
        self
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum MagnificationFilter {
    /// Pixellated
    Nearest,
    /// Blurred
    Linear,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum MinificationFilter {
    /// Single pixel color lookup
    Nearest,
    /// Mixes colors of pixels
    Linear,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum MipmapLevel {
    /// Use no mipmaps,
    Off,
    /// Use single mipmap lookup, may create sharp transitions between different minification degrees.
    Single,
    /// Linear interpolation between two mipmap lookups
    Double,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum ColorFormat {
    /// Red, Green, and Blue
    RGB,
    /// Red, Green, Blue, and Alpha
    RGBA,
}

impl MipmapLevel {
    pub fn on(&self) -> bool {
        *self != MipmapLevel::Off
    }
}
impl MagnificationFilter {
    pub(crate) fn webgl_num(&self) -> i32 {
        (match self {
            MagnificationFilter::Nearest => WebGlRenderingContext::NEAREST,
            MagnificationFilter::Linear => WebGlRenderingContext::LINEAR,
        }) as i32
    }
}
impl MinificationFilter {
    pub(crate) fn webgl_num(&self, mipmap: &MipmapLevel) -> i32 {
        (match (self, mipmap) {
            (MinificationFilter::Nearest, MipmapLevel::Off) => WebGlRenderingContext::NEAREST,
            (MinificationFilter::Linear, MipmapLevel::Off) => WebGlRenderingContext::LINEAR,
            (MinificationFilter::Nearest, MipmapLevel::Single) => {
                WebGlRenderingContext::NEAREST_MIPMAP_NEAREST
            }
            (MinificationFilter::Linear, MipmapLevel::Single) => {
                WebGlRenderingContext::LINEAR_MIPMAP_NEAREST
            }
            (MinificationFilter::Nearest, MipmapLevel::Double) => {
                WebGlRenderingContext::NEAREST_MIPMAP_LINEAR
            }
            (MinificationFilter::Linear, MipmapLevel::Double) => {
                WebGlRenderingContext::LINEAR_MIPMAP_LINEAR
            }
        }) as i32
    }
}
impl ColorFormat {
    pub(crate) fn webgl_num(&self) -> i32 {
        (match self {
            ColorFormat::RGB => WebGlRenderingContext::RGB,
            ColorFormat::RGBA => WebGlRenderingContext::RGBA,
        }) as i32
    }
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            magnification_filter: MagnificationFilter::Linear,
            minification_filter: MinificationFilter::Linear,
            mipmap_level: MipmapLevel::Single,
            color_format: ColorFormat::RGBA,
        }
    }
}
