use crate::{
    ComplexShape, Image, JsError, LoadActivity, LoadedImageAsset, LoadedShapeAsset, PaddleResult,
};

/// Image descriptor: Names an image is loaded and can be used for drawing.
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct ImageDesc {
    path: &'static str,
}

/// Shape descriptor: Names a complex shape and can be used for drawing.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

    /// Use if you've somehow gotten a PNG without reading it from a URL.
    pub fn from_png_binary(raw: &[u8]) -> PaddleResult<ImageDesc> {
        let typed_array = js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(raw) }.into());
        let array = js_sys::Array::new();
        array.push(&typed_array.buffer());
        let blob =
            web_sys::Blob::new_with_u8_array_sequence(&array).map_err(JsError::from_js_value)?;
        let download_url =
            web_sys::Url::create_object_url_with_blob(&blob).map_err(JsError::from_js_value)?;
        Ok(ImageDesc::from_path(Box::leak(
            download_url.into_boxed_str(),
        )))
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
