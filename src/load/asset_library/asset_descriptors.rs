use crate::{ComplexShape, Image, LoadActivity, LoadedImageAsset, LoadedShapeAsset, PaddleResult};

/// Image descriptor: Names an image is loaded and can be used for drawing.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ImageDesc {
    path: &'static str,
}

/// Shape descriptor: Names a complex shape and can be used for drawing.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ShapeDesc {
    pub(crate) name: &'static str,
}

impl ImageDesc {
    pub const fn from_path(path: &'static str) -> Self {
        Self { path }
    }
    /// Creates a future that loads the specified image and hands it to the asset library.
    /// Usually an `AssetBundle` is the better choice rather than calling this function directly.
    pub async fn load(&self) -> PaddleResult<()> {
        Image::load(self.path)
            .await
            .map(|img| nuts::send_to::<LoadActivity, _>(LoadedImageAsset { desc: *self, img }))
    }
}

impl ShapeDesc {
    pub const fn named(name: &'static str) -> Self {
        Self { name }
    }
    /// Define the shape and store it in the asset library.
    pub fn define(&self, shape: ComplexShape) {
        nuts::send_to::<LoadActivity, _>(LoadedShapeAsset { desc: *self, shape })
    }
}
