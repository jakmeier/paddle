use web_sys::WebGlTexture;

use crate::{
    graphics::Image,
    quicksilver_compat::geom::{Rectangle, Transform, Vector},
};
use std::rc::Rc;

///Pixel formats for use with loading raw images
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PixelFormat {
    /// Red, Green, and Blue
    RGB,
    /// Red, Green, Blue, and Alpha
    RGBA,
}

// impl Drop for ImageData {
//     fn drop(&mut self) {
//         unsafe { instance().destroy_texture(self) };
//     }
// }

impl Image {
    // pub(crate) fn new(data: ImageData) -> Image {
    //     let region = Rectangle::new_sized((data.width, data.height));
    //     Image {
    //         source: Rc::new(data),
    //         region
    //     }
    // }

    // /// Start loading a texture from a given path
    // pub fn load<P: AsRef<Path>>(path: P) -> impl Future<Item = Image, Error = QuicksilverError> {
    //     load_file(path)
    //         .map(|data| Image::from_bytes(data.as_slice()))
    //         .and_then(future::result)
    // }

    // pub(crate) fn new_null(width: u32, height: u32, format: PixelFormat) -> Result<Image> {
    //     Image::from_raw(&[], width, height, format)
    // }

    // /// Load an image from pixel values in a byte array
    // pub fn from_raw(data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<Image> {
    //     Ok(unsafe {
    //         Image::new(instance().create_texture(data, width, height, format)?)
    //     })
    // }

    // /// Load an image directly from an encoded byte array
    // pub fn from_bytes(raw: &[u8]) -> Result<Image> {
    //     let img = image::load_from_memory(raw)?.to_rgba();
    //     let width = img.width();
    //     let height = img.height();
    //     Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA)
    // }

    // pub(crate) fn get_id(&self) -> u32 {
    //     self.source.id
    // }

    pub(crate) fn source_width(&self) -> i32 {
        self.source.width
    }

    pub(crate) fn source_height(&self) -> i32 {
        self.source.height
    }

    ///The area of the source image this subimage takes up
    pub fn area(&self) -> Rectangle {
        self.region
    }

    ///Find a subimage of a larger image
    pub fn subimage(&self, rect: Rectangle) -> Image {
        Image {
            source: self.source.clone(),
            region: Rectangle::new(
                (
                    self.region.pos.x + rect.pos.x,
                    self.region.pos.y + rect.pos.y,
                ),
                (rect.width(), rect.height()),
            ),
        }
    }

    /// Create a projection matrix for a given region onto the Image
    pub fn projection(&self, region: Rectangle) -> Transform {
        let source_size: Vector = (self.source_width(), self.source_height()).into();
        let recip_size = source_size.recip();
        let normalized_pos = self.region.top_left().times(recip_size);
        let normalized_size = (1.0, -1.0); // self.region.size().times(recip_size);
        Transform::translate(normalized_pos)
            * Transform::scale(normalized_size)
            * Transform::scale(region.size().recip())
            * Transform::translate(-region.top_left())
    }

    pub(crate) fn texture(&self) -> &WebGlTexture {
        &self.source.tex.webgl_texture()
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) && self.region == other.region
    }
}
impl Eq for Image {}
