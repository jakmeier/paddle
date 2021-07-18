use crate::{Image, LoadActivity, LoadedImageAsset, PaddleResult};

/// Image descriptor: Describes how an image is loaded and can be used for drawing.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ImageDesc {
    path: &'static str,
}

impl ImageDesc {
    pub const fn new(path: &'static str) -> Self {
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
