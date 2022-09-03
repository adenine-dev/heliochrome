const WIDTH: u16 = 640;
const HEIGHT: u16 = 360;

use crate::heliochrome::{context::Context, maths::*, *};

pub fn make_context() -> Context {
    let mut context = Context::new(
        maths::Size::new(WIDTH, HEIGHT),
        camera::Camera::new(
            maths::vec3::new(0.0, 0.0, 0.0),
            WIDTH as f32 / HEIGHT as f32,
        ),
    );

    context.add_hittable(objects::Sphere::new(vec3::new(0.0, 0.0, -1.0), 0.5));
    context.add_hittable(objects::Sphere::new(vec3::new(0.0, -100.5, -1.0), 100.0));

    context
}
