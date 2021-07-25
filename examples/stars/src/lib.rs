use lyon::{math::point, path::Path, tessellation::*};
use paddle::quicksilver_compat::*;
use paddle::*;
use rand::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

/*
    This example shows how to create a custom shape and draw it in different sizes and colors.
*/

#[wasm_bindgen]
pub fn start() {
    // Build configuration object to define all setting
    let config = PaddleConfig::default()
        .with_canvas_id("paddle-canvas-id")
        .with_background_color(BACKGROUND_COLOR)
        .with_resolution((1400, 1050));

    // Start game engine
    paddle::init(config).expect("Paddle initialization failed.");

    // Create a shape programmatically and store it in the asset library.
    let shape = build_star_shape();
    STAR_SHAPE.define(shape);

    // Create our game state and register it as the only frame
    let global_state = ();
    paddle::register_frame(Game::default(), global_state, (0, 0));
}

#[derive(Default)]
struct Game {
    stars: Vec<Star>,
    rot_idx: usize,
}

struct Star {
    position: Vector,
    size: Vector,
    color: Color,
    period: f64,
}

impl paddle::Frame for Game {
    type State = ();
    const WIDTH: u32 = 1400;
    const HEIGHT: u32 = 1050;

    fn draw(&mut self, _state: &mut Self::State, canvas: &mut DisplayArea, timestamp: f64) {
        canvas.fit_display(10.0);

        let z = 1;

        for star in &self.stars {
            let pulse = f64::sin(star.period * timestamp);
            let size = star.size + star.size * pulse * 0.125;
            let transform = Transform::translate(star.position) * Transform::scale(size);
            canvas.draw_ex(
                &STAR_SHAPE,
                &star.color.with_alpha(1.0 - 0.25 + 0.25 * pulse as f32),
                transform,
                z,
            );
        }
    }

    fn update(&mut self, _state: &mut Self::State) {
        let mut rng = rand::thread_rng();
        let s: f32 = rng.gen();
        let px: f32 = rng.gen();
        let py: f32 = rng.gen();
        let cr: f32 = rng.gen();
        let cg: f32 = rng.gen();
        let cb: f32 = rng.gen();
        let p: f64 = rng.gen();

        let sf = 0.1875;
        let cf = 0.25;
        let new_star = Star {
            position: Vector::new(Self::WIDTH as f32 * px, Self::HEIGHT as f32 * py),
            size: Vector::new(0.03125 + s * sf, 0.03125 + s * sf),
            color: Color::new(1.0 - cf * cr, 1.0 - cf * cg, 1.0 - cf * cb),
            period: p / 300.0,
        };

        if self.stars.len() < 500 {
            self.stars.push(new_star);
        } else {
            self.stars[self.rot_idx] = new_star;
            self.rot_idx = (self.rot_idx + 1) % self.stars.len();
        }
    }
}

const BACKGROUND_COLOR: Color = Color {
    r: 0.05,
    g: 0.05,
    b: 0.15,
    a: 1.0,
};

const STAR_SHAPE: ShapeDesc = ShapeDesc::named("my star");

fn build_star_shape() -> ComplexShape {
    // Creating a new shape programmatically requires these steps:
    // 1) Draw the shape using lyon
    // 2) Tesselate it into triangles

    // This example uses quadratic bezier-curves for drawing the shape.
    // Each curve is defined by its two endpoints and a control point.
    // The control point is somewhere between the two endpoints but is never actually reached by the drawn curve.
    // See wikipedia for how exactly this works: https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Quadratic_B%C3%A9zier_curves

    // These are the four corner of the star
    let north = point(0.0, -100.0);
    let east = point(100.0, 0.0);
    let south = point(0.0, 100.0);
    let west = point(-100.0, 0.0);

    // The control point is the center point.
    let ctrl = point(0.0, 0.0);

    // Create enclosing path
    let mut builder = Path::builder();

    builder.begin(north);
    builder.quadratic_bezier_to(ctrl, east);
    builder.quadratic_bezier_to(ctrl, south);
    builder.quadratic_bezier_to(ctrl, west);
    builder.quadratic_bezier_to(ctrl, north);
    builder.close();

    let path = builder.build();

    // Tesselate path to mesh
    let mut mesh = AbstractMesh::new();
    let mut tessellator = FillTessellator::new();
    let mut shape = ShapeRenderer::new(&mut mesh);

    tessellator
        .tessellate_path(&path, &FillOptions::default(), &mut shape)
        .unwrap();

    // The bounding box is used for sizing the shape properly.
    // The pivot point, however, is always (0,0). The pivot point is used to position the shape.
    let bounding_box = Rectangle::new((-100, -100), (200, 200));
    ComplexShape::new(mesh, bounding_box)
}
