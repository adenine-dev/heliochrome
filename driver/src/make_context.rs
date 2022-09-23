const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

use std::path::Path;

use heliochrome::{
    color::Color, context::Context, hittables, image::Image, load_obj::load_obj, materials::*,
    maths::*, object::Object, scene::Scene, transform::Transform, *,
};

pub fn make_context() -> Context {
    let mut scene = Scene::new(
        camera::Camera::new(
            maths::vec3::new(0.0, 0.0, 5.0),
            vec3::new(0.0, 0.0, 0.0),
            vec3::unit_y(),
            20.0,
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            None,
        )
        .into(),
    );
    scene.skybox =
        Some(Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap());

    scene.add_object(Object::new(
        hittables::InfinitePlane::new(
            vec3::new(0.0, -1.0, 0.0),
            vec3::new(0.0, 1.0, 0.0).normalized(),
        )
        .into(),
        Lambertian::new(Color::splat(0.3)).into(),
        None,
    ));

    // let (mut positions, indices) = load_obj("assets/suzanne.obj").unwrap();
    // positions.iter_mut().for_each(|p| {
    //     *p -= vec3::new(0.0, 0.55, 0.0);
    // });
    // scene.add_object(Object::new(
    //     hittables::Mesh::new(&positions, &indices).into(),
    //     Metal::new(Color::new(0.97, 0.77, 0.06), 0.0).into(),
    //     // Dielectric::new(1.5, Color::new(0.8, 0.3, 0.8)).into(),
    //     // None,
    //     Some(Transform::new(mat3::rotate(vec3::new(
    //         0.0,
    //         0.0,
    //         -35.0f32.to_radians(),
    //     )))),
    // ));

    scene.add_object(Object::new(
        hittables::Sphere::new(vec3::new(-1.0, 0.0, -1.0), 0.95).into(),
        Metal::new(Color::new(0.8, 0.8, 0.8), 0.0).into(),
        None,
    ));

    scene.add_object(Object::new(
        hittables::Sphere::new(vec3::new(1.0, 0.0, -1.0), 0.95).into(),
        Lambertian::new(Color::new(0.1, 0.1, 0.1)).into(),
        None,
    ));

    scene.add_object(Object::new(
        hittables::Sphere::new(vec3::new(1.0, 0.0, 1.0), 0.95).into(),
        Metal::new(Color::new(0.3, 0.3, 0.3), 0.7).into(),
        None,
    ));

    scene.add_object(Object::new(
        hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), 0.95).into(),
        Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
        None,
    ));
    scene.add_object(Object::new(
        hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), -0.85).into(),
        Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
        None,
    ));

    Context::new(maths::vec2::new(WIDTH, HEIGHT), scene)

    // let mut objects = vec![];
    // for x in 0..5 {
    //     for y in 0..5 {
    //         for z in 0..5 {
    //             let p = vec3::new(x as f32 - 2.0, y as f32 - 2.0, z as f32 - 2.0);
    //             objects.push(Object::new(
    //                 hittables::AABB::new(vec3::splat(-0.35) + p, vec3::splat(0.35) + p).into(),
    //                 Lambertian::new(
    //                     Color::new(x as f32 / 5.0, y as f32 / 5.0, z as f32 / 5.0) * 0.8
    //                         + Color::splat(0.1),
    //                     // 0.1,
    //                 )
    //                 .into(),
    //                 None,
    //             ));
    //         }
    //     }
    // }

    // positions.iter_mut().for_each(|p| {
    //     *p += vec3::new(1.0, 0.0, 0.0);
    // });

    // context.add_object(Object::new(
    //     hittables::Mesh::new(&positions, &indices).into(),
    //     Metal::new(Color::new(0.5, 0.5, 0.9), 0.0).into(),
    //     None,
    // ));

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
}
