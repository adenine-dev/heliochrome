const WIDTH: u16 = 400;
const HEIGHT: u16 = 225;

use std::path::Path;

use crate::{
    heliochrome::{
        color::Color, context::Context, hittables, image::Image, materials::*, maths::*,
        object::Object, transform::Transform, *,
    },
    load_obj::load_obj,
};

pub fn make_context() -> Context {
    let mut context = Context::new(
        maths::Size::new(WIDTH, HEIGHT),
        camera::Camera::new(
            maths::vec3::new(0.0, 0.0, 3.0),
            vec3::new(1.0, -1.0, 0.0),
            vec3::unit_y(),
            20.0,
            WIDTH as f32 / HEIGHT as f32,
            0.1,
        )
        .into(),
    );

    context.skybox =
        Some(Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap());

    context.add_object(Object::new(
        hittables::InfinitePlane::new(
            vec3::new(0.0, -3.0, 0.0),
            vec3::new(0.0, 1.0, 0.0).normalized(),
        )
        .into(),
        Lambertian::new(Color::splat(0.5)).into(),
        None,
    ));

    let (mut positions, indices) = load_obj("assets/torus.obj").unwrap();
    context.add_object(Object::new(
        hittables::Mesh::new(&positions, &indices).into(),
        Metal::new(Color::new(0.8, 0.3, 0.8), 0.0).into(),
        Some(Transform::new(mat3::rotate(vec3::new(
            0.0,
            0.0,
            std::f32::consts::TAU / 4.0,
        )))),
    ));

    positions.iter_mut().for_each(|p| {
        *p += vec3::new(1.0, 0.0, 0.0);
    });

    context.add_object(Object::new(
        hittables::Mesh::new(&positions, &indices).into(),
        Metal::new(Color::new(0.5, 0.5, 0.9), 0.0).into(),
        None,
        // Some(Transform::new(mat3::rotate(vec3::new(
        //     0.0,
        //     0.0,
        //     std::f32::consts::TAU / 8.0,
        // )))),
    ));

    // context.add_object(Object::new(
    //     hittables::Triangle::new([
    //         vec3::new(2.179167, -0.818439, -1.094899),
    //         vec3::new(2.145673, -0.693439, -1.094899),
    //         vec3::new(2.054167, -0.601933, -1.094899),
    //     ])
    //     .into(),
    //     Metal::new(Color::new(0.8, 0.3, 0.8), 0.1).into(),
    //     None,
    // ));

    // context.add_object(
    //     hittables::Rect::new(
    //         vec3::new(0.0, 0.0, 0.0),
    //         vec3::new(-1.0, (2.0f32).sqrt(), -1.0),
    //         vec3::new(1.0, (2.0f32).sqrt(), 1.0),
    //         Metal::new(Color::new(1.0, 0.2, 0.2), 0.5).into(),
    //     )
    //     .into(),
    // );

    // context.add_object(
    //     hittables::Sphere::new(
    //         vec3::new(0.0, 0.0, -1.0),
    //         0.5,
    //         Lambertian::new(Color::new(0.8, 0.3, 1.0)).into(),
    //     )
    //     .into(),
    // );

    // context.add_object(
    //     hittables::Sphere::new(
    //         vec3::new(-1.0, 0.0, -1.0),
    //         -0.4,
    //         Dielectric::new(1.5, Color::splat(1.0)).into(),
    //     )
    //     .into(),
    // );

    // context.add_object(
    //     hittables::Sphere::new(
    //         vec3::new(-1.0, 0.0, -1.0),
    //         0.5,
    //         Dielectric::new(1.5, Color::splat(1.0)).into(),
    //     )
    //     .into(),
    // );

    // context.add_object(
    //     hittables::Sphere::new(
    //         vec3::new(1.0, 0.0, -1.0),
    //         0.5,
    //         Metal::new(Color::new(0.3, 0.3, 0.7), 0.3).into(),
    //     )
    //     .into(),
    // );

    // context.add_object(Object::new(
    //     hittables::Sphere::new(vec3::new(0.0, 0.0, 0.0), 0.95).into(),
    //     Metal::new(Color::new(0.8, 0.8, 0.8), 0.0).into(),
    //     Some(Transform::new(
    //         mat3::rotate(vec3::new(std::f32::consts::TAU / 3.0, 0.0, 0.0))
    //             * mat3::scale(vec3::new(1.0, 2.0, 1.0)),
    //     )),
    // ));

    // context.add_object(Object::new(
    //     hittables::Sphere::new(vec3::new(1.0, 0.0, -1.0), 0.95).into(),
    //     Lambertian::new(Color::new(0.1, 0.1, 0.1)).into(),
    //     None,
    // ));

    // context.add_object(Object::new(
    //     hittables::Sphere::new(vec3::new(1.0, 0.0, 1.0), 0.95).into(),
    //     Metal::new(Color::new(0.3, 0.3, 0.3), 0.7).into(),
    //     None,
    // ));

    // context.add_object(Object::new(
    //     hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), 0.95).into(),
    //     Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
    //     None,
    // ));
    // context.add_object(Object::new(
    //     hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), -0.85).into(),
    //     Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
    //     None,
    // ));

    context
}
