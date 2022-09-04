const WIDTH: u16 = 640;
const HEIGHT: u16 = 360;

use crate::heliochrome::{color::Color, context::Context, materials::Lambertian, maths::*, *};

pub fn make_context() -> Context {
    let mut context = Context::new(
        maths::Size::new(WIDTH, HEIGHT),
        camera::Camera::new(
            maths::vec3::new(0.0, 0.0, 0.0),
            WIDTH as f32 / HEIGHT as f32,
        ),
    );

    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Color::new(0.8, 0.3, 1.0)).into(),
        )
        .into(),
    );
    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Color::new(0.5, 0.5, 0.5)).into(),
        )
        .into(),
    );

    context
}
