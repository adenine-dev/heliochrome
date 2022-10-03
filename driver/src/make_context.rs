// const WIDTH: f32 = 1280.0;
// const HEIGHT: f32 = 720.0;
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 345.0;

use std::path::Path;

use heliochrome::{
    color::Color,
    context::Context,
    hittables::{self},
    image::Image,
    load_obj::load_obj,
    materials::*,
    maths::*,
    object::Object,
    scene::{Scene, SkyBox},
    sdf::SDF,
    tonemap::ToneMap,
    transform::Transform,
    *,
};

#[allow(clippy::vec_init_then_push)]
pub fn make_context() -> Context {
    let mut objects = vec![];
    // objects.push(Object::new(
    //     hittables::InfinitePlane::new(
    //         vec3::new(0.0, 0.0, 0.0),
    //         vec3::new(0.0, 1.0, 0.0).normalized(),
    //     )
    //     .into(),
    //     Lambertian::new(Color::splat(0.3)).into(),
    //     None,
    // ));

    // let mut meshes = load_obj("assets/suzanne.obj").unwrap();
    // let mesh = meshes.first_mut().unwrap();
    // mesh.vertices.iter_mut().for_each(|p| {
    //     *p -= vec3::new(0.0, -0.55, 0.0);
    // });
    // objects.push(Object::new(
    //     hittables::Mesh::new(&mesh.vertices, &mesh.indices).into(),
    //     Metal::new(Color::new(1.0, 0.9, 1.0), 0.7).into(),
    //     // None,
    //     Some(Transform::new(mat3::rotate(vec3::new(
    //         0.0,
    //         0.0,
    //         -35.0f32.to_radians(),
    //     )))),
    // ));

    // objects.push(Object::new(
    //     // hittables::AABB::new(vec3::new(1.0, -1.0, -1.0), vec3::new(3.0, 1.0, 1.0)).into(),
    //     hittables::AABB::new(vec3::splat(-1.0), vec3::splat(1.0)).into(),
    //     Dielectric::new(1.5, Color::new(0.9, 1.0, 1.0)).into(),
    //     // None,
    //     Some(Transform::new(
    //         mat3::rotate(vec3::splat(std::f32::consts::TAU / 8.0)) * mat3::scale(vec3::splat(0.3)),
    //     )),
    // ));

    // objects.push(Object::new(
    //     hittables::Sphere::new(vec3::new(0.0, 3.0, 0.0), 0.4).into(),
    //     DiffuseLight::new(Color::splat(1.0), 4.0).into(),
    //     None,
    // ));

    // objects.push(Object::new(
    //     hittables::Sphere::new(vec3::new(-1.0, 0.0, -1.0), 0.95).into(),
    //     Metal::new(Color::new(0.8, 0.8, 0.8), 0.0).into(),
    //     None,
    // ));

    // objects.push(Object::new(
    //     hittables::Sphere::new(vec3::new(1.0, 0.0, -1.0), 0.95).into(),
    //     Lambertian::new(Color::new(0.1, 0.1, 0.1)).into(),
    //     None,
    // ));

    // objects.push(Object::new(
    //     hittables::Sphere::new(vec3::new(1.0, 0.0, 1.0), 0.95).into(),
    //     Metal::new(Color::new(0.3, 0.3, 0.3), 0.7).into(),
    //     None,
    // ));

    // objects.push(Object::new(
    //     hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), 0.95).into(),
    //     Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
    //     None,
    // ));
    // objects.push(Object::new(
    //     hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), -0.85).into(),
    //     Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)).into(),
    //     None,
    // ));

    // let n = 5;
    // for x in 0..n {
    //     for y in 0..n {
    //         for z in 0..n {
    //             let p = vec3::new(
    //                 x as f32 - n as f32 / 2.0,
    //                 y as f32 - n as f32 / 2.0,
    //                 z as f32 - n as f32 / 2.0,
    //             );
    //             objects.push(Object::new(
    //                 hittables::AABB::new(vec3::splat(-0.35) + p, vec3::splat(0.35) + p).into(),
    //                 Lambertian::new(
    //                     Color::new(
    //                         x as f32 / n as f32,
    //                         y as f32 / n as f32,
    //                         z as f32 / n as f32,
    //                     ) * 0.8
    //                         + Color::splat(0.1),
    //                     // 0.1,
    //                 )
    //                 .into(),
    //                 // None,
    //                 Some(Transform::new(mat3::rotate(vec3::splat(
    //                     std::f32::consts::TAU / 8.0,
    //                 )))),
    //             ));
    //         }
    //     }
    // }

    // objects.push(Object::new(
    //     hittables::HittableSDF::new(sdf::Sphere::new(0.2, vec3::new(-0.25, -0.25, 0.25))).into(),
    //     Lambertian::new(Color::new(0.9, 0.3, 0.6)).into(),
    //     None,
    // ));

    // objects.push(Object::new(
    //     hittables::HittableSDF::new(sdf::Sphere::new(0.2, vec3::new(0.25, 0.25, -0.25))).into(),
    //     Lambertian::new(Color::new(0.9, 0.3, 0.6)).into(),
    //     None,
    // ));

    // objects.push(Object::new(
    //     hittables::HittableSDF::new(
    //         sdf::Sphere::new(0.2, vec3::splat(0.25))
    //             .smooth_union(1.0, sdf::Sphere::new(0.2, vec3::splat(-0.25))),
    //     )
    //     .into(),
    //     Metal::new(Color::new(0.3, 0.9, 0.6), 0.1).into(),
    //     None,
    // ));

    objects.push(Object::new(
        hittables::HittableSDF::new(
            sdf::Torus::new(1.0, 0.5)
                .smooth_difference(0.1, sdf::Sphere::new(0.5, -vec3::unit_x()))
                .smooth_difference(0.1, sdf::Sphere::new(0.5, vec3::unit_x()))
                .smooth_difference(0.1, sdf::Sphere::new(0.5, -vec3::unit_z()))
                .smooth_difference(0.1, sdf::Sphere::new(0.5, vec3::unit_z())),
        )
        .into(),
        Metal::new(Color::splat(0.4), 0.3).into(),
        None, // Some(Transform::new(mat3::rotate(vec3::new(
              //     0.0,
              //     0.0,
              //     std::f32::consts::TAU / 4.0,
              // )))),
    ));

    let scene = Scene::new(
        camera::Camera::new(
            maths::vec3::new(0.0, 0.0, 4.0),
            vec3::new(0.0, 0.0, 0.0),
            vec3::unit_y(),
            60.0,
            WIDTH as f32 / HEIGHT as f32,
            0.0,
            None,
        ),
        // SkyBox::Color(Color::new(0.0, 0.0, 0.0)),
        SkyBox::Equirectangular(
            Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap(),
        ),
        objects,
    );

    Context::new(maths::vec2::new(WIDTH, HEIGHT), scene, ToneMap::HejlRichard)

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
