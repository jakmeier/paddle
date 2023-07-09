use crate::{AbstractMesh, Rectangle, Tessellate, Transform};

/// A single mesh of triangles ready to be drawn
pub struct ComplexShape {
    /// Normalized mesh defining the shape
    mesh: AbstractMesh,
    /// Natural size used for drawing without scaling
    bounding_box: Rectangle,
}

impl ComplexShape {
    pub fn new(mut mesh: AbstractMesh, bounding_box: Rectangle) -> Self {
        mesh.normalize(&bounding_box);
        Self { mesh, bounding_box }
    }

    pub fn from_shape(shape: impl Tessellate) -> Self {
        let mut mesh = AbstractMesh::new();
        let bounding_box = shape.bounding_box();
        shape.tessellate(&mut mesh);
        Self { mesh, bounding_box }
    }

    pub fn rounded_rectangle(bounding_box: Rectangle, radius: f32) -> Self {
        // build the lyon path
        let path = {
            let radii = lyon::path::builder::BorderRadii::new(radius);
            let winding = lyon::path::Winding::Positive;
            let mut builder = lyon::path::Path::builder();
            builder.add_rounded_rectangle(&bounding_box.into(), &radii, winding);

            builder.build()
        };

        // now create triangles
        let mut mesh = AbstractMesh::new();
        let mut shape = crate::ShapeRenderer::new(&mut mesh);

        lyon::lyon_tessellation::FillTessellator::new()
            .tessellate_path(
                &path,
                &lyon::lyon_tessellation::FillOptions::default(),
                &mut shape,
            )
            .unwrap();

        mesh.normalize(&bounding_box);
        Self { mesh, bounding_box }
    }

    pub fn resize(&mut self, bounding_box: &Rectangle) {
        self.bounding_box = *bounding_box;
    }

    pub fn transform(&self) -> Transform {
        Rectangle::new((-1, -1), (2, 2)).project(&self.bounding_box)
    }
}

impl Tessellate for ComplexShape {
    fn tessellate<'a>(&self, mesh: &mut AbstractMesh) {
        mesh.extend(&self.mesh);
    }

    fn bounding_box(&self) -> Rectangle {
        self.bounding_box
    }
}
