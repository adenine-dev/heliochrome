const WIDTH: u16 = 400;
const HEIGHT: u16 = 225;

use crate::heliochrome::{color::Color, context::Context, materials::*, maths::*, *};

pub fn make_context() -> Context {
    let mut context = Context::new(
        maths::Size::new(WIDTH, HEIGHT),
        camera::Camera::new(
            maths::vec3::new(-2.0, 2.0, 1.0),
            maths::vec3::new(0.0, 0.0, 0.0),
            vec3::unit_y(),
            90.0,
            WIDTH as f32 / HEIGHT as f32,
        ),
    );

    // context.add_hittable(
    //     hittables::Sphere::new(
    //         vec3::new(0.0, 0.0, -1.0),
    //         0.5,
    //         Lambertian::new(Color::new(0.8, 0.3, 1.0)).into(),
    //     )
    //     .into(),
    // );

    // context.add_hittable(
    //     hittables::Sphere::new(
    //         vec3::new(-1.0, 0.0, -1.0),
    //         -0.4,
    //         Dielectric::new(1.5).into(),
    //     )
    //     .into(),
    // );

    // context.add_hittable(
    //     hittables::Sphere::new(vec3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5).into()).into(),
    // );

    // context.add_hittable(
    //     hittables::Sphere::new(
    //         vec3::new(1.0, 0.0, -1.0),
    //         0.5,
    //         Metal::new(Color::new(0.3, 0.3, 0.7), 0.3).into(),
    //     )
    //     .into(),
    // );

    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(-1.0, 0.0, -1.0),
            0.95,
            Metal::new(Color::new(0.8, 0.8, 0.8), 0.0).into(),
        )
        .into(),
    );

    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(1.0, 0.0, -1.0),
            0.95,
            Lambertian::new(Color::new(0.1, 0.1, 0.1)).into(),
        )
        .into(),
    );

    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(1.0, 0.0, 1.0),
            0.95,
            Metal::new(Color::new(0.3, 0.3, 0.3), 0.7).into(),
        )
        .into(),
    );

    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(-1.0, 0.0, 1.0),
            0.95,
            Dielectric::new(1.2, Color::new(1.0, 0.3, 0.3)).into(),
        )
        .into(),
    );
    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(-1.0, 0.0, 1.0),
            -0.85,
            Dielectric::new(1.2, Color::new(1.0, 0.3, 0.3)).into(),
        )
        .into(),
    );

    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(0.0, -1000.5, -1.0),
            1000.0,
            Lambertian::new(Color::new(0.5, 0.5, 0.5)).into(),
        )
        .into(),
    );

    context
}
