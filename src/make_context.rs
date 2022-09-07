const WIDTH: u16 = 400;
const HEIGHT: u16 = 225;

use std::path::Path;

use crate::heliochrome::{
    color::Color, context::Context, hittables, image::Image, materials::*, maths::*, *,
};

pub fn make_context() -> Context {
    let mut context = Context::new(
        maths::Size::new(WIDTH, HEIGHT),
        camera::Camera::new(
            maths::vec3::new(3.0, 3.0, 2.0),
            maths::vec3::new(-1.0, 0.0, 1.0),
            vec3::unit_y(),
            20.0,
            WIDTH as f32 / HEIGHT as f32,
            0.1,
        )
        .into(),
    );

    context.skybox =
        Some(Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap());

    context.add_hittable(
        hittables::InfinitePlane::new(
            vec3::new(0.0, -1.1, 0.0),
            vec3::new(0.0, 1.0, 0.0).normalized(),
            Lambertian::new(Color::splat(0.5)).into(),
        )
        .into(),
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
    //         Dielectric::new(1.5, Color::splat(1.0)).into(),
    //     )
    //     .into(),
    // );

    // context.add_hittable(
    //     hittables::Sphere::new(
    //         vec3::new(-1.0, 0.0, -1.0),
    //         0.5,
    //         Dielectric::new(1.5, Color::splat(1.0)).into(),
    //     )
    //     .into(),
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
            Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
        )
        .into(),
    );
    context.add_hittable(
        hittables::Sphere::new(
            vec3::new(-1.0, 0.0, 1.0),
            -0.85,
            Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
        )
        .into(),
    );

    // context.add_hittable(
    //     hittables::Sphere::new(
    //         vec3::new(0.0, -1000.5, -1.0),
    //         1000.0,
    //         Lambertian::new(Color::new(0.5, 0.5, 0.5)).into(),
    //     )
    //     .into(),
    // );

    context
}
