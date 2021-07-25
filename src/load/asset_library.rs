//! The asset library manages media objects.
//!
//! It makes loading allows to load many assets in parallel and will then store them in the background.
//! Each DisplayFrame has access to the global asset library, thus once media is loaded, it can be used by all frames for drawing on the screen etc.
//!
//! TODO: Example

pub mod asset_descriptors;
pub mod asset_loading;

pub use asset_descriptors::*;
pub use asset_loading::*;
use nuts::DomainState;

use crate::{
    AbstractMesh, ComplexShape, Context, DisplayPaint, DisplayTessellate, ErrorMessage, Image,
    NutsCheck, PaddleResult, Rectangle, Tessellate,
};
use std::collections::HashMap;

/// Internal storage of assets loaded through an `AssetBundle`
#[derive(Default)]
pub struct AssetLibrary {
    images: HashMap<ImageDesc, Image>,
    // animations: Vec<AnimatedObject>,
    shapes: HashMap<ShapeDesc, ComplexShape>,
}

impl AssetLibrary {
    pub fn add_image(&mut self, desc: ImageDesc, img: Image) {
        self.images.insert(desc, img);
    }
    pub fn add_shape(&mut self, desc: ShapeDesc, shape: ComplexShape) {
        self.shapes.insert(desc, shape);
    }
    pub(crate) fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        context.display.full_mut().asset_library()
    }
    pub fn lookup_shape(&self, desc: ShapeDesc) -> PaddleResult<&ComplexShape> {
        self.shapes.get(&desc).ok_or_else(|| {
            ErrorMessage::technical(format!(
                "Attempted to draw ComplexShape that has not been laded: {}",
                desc.name
            ))
        })
    }
}

impl DisplayPaint for ImageDesc {
    fn image<'a>(&'a self, assets: &'a AssetLibrary) -> Option<&'a Image> {
        assets.images.get(self)
    }
}

impl DisplayTessellate for ShapeDesc {
    fn tessellate(&self, assets: &AssetLibrary, mesh: &mut AbstractMesh) {
        if let Some(shape) = assets.lookup_shape(*self).nuts_check() {
            Tessellate::tessellate(shape, mesh);
        }
    }

    fn bounding_box(&self, assets: &AssetLibrary) -> PaddleResult<Rectangle> {
        assets.lookup_shape(*self).map(Tessellate::bounding_box)
    }
}
