use crate::{AbstractMesh, AssetLibrary, NutsCheck, PaddleResult, Rectangle, Tessellate};

/// Implementor of this trait can be used on `Display` and `DisplayArea` to define geometric shapes.
///
/// This trait is similar to `Tessellate`. But it also has access the asset library on the display.
pub trait DisplayTessellate {
    fn tessellate(&self, assets: &AssetLibrary, mesh: &mut AbstractMesh);
    fn bounding_box(&self, assets: &AssetLibrary) -> PaddleResult<Rectangle>;
}

/// A tuple of an asset library with anything that implements `Tessellate` also implements `Tessellate`.
/// This allows descriptors to be used for rendering.
impl<DP: DisplayTessellate> Tessellate for (&DP, &AssetLibrary) {
    fn tessellate(&self, mesh: &mut AbstractMesh) {
        DisplayTessellate::tessellate(self.0, &self.1, mesh)
    }
    fn bounding_box(&self) -> Rectangle {
        DisplayTessellate::bounding_box(self.0, &self.1)
            .nuts_check()
            .unwrap_or(crate::ABSTRACT_SPACE)
    }
}

// Anything that implements `Tessellate` also implements `DisplayTessellate`.
// This allows normal shapes (not part of asset library) to be used on the display draw methods.
impl<P: Tessellate> DisplayTessellate for P {
    fn tessellate(&self, _assets: &AssetLibrary, mesh: &mut AbstractMesh) {
        Tessellate::tessellate(self, mesh)
    }
    fn bounding_box(&self, _assets: &AssetLibrary) -> PaddleResult<Rectangle> {
        Ok(Tessellate::bounding_box(self))
    }
}
