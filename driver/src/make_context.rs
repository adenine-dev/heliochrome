// const WIDTH: f32 = 1280.0;
// const HEIGHT: f32 = 720.0;
// const WIDTH: f32 = 800.0;
// const HEIGHT: f32 = 345.0;
const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 300.0;

use std::path::Path;

use heliochrome::{
    camera::Camera,
    color::Color,
    context::Context,
    hittables::{self, Hittable},
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
    let mut camera = Camera::new(
        maths::vec3::new(3.0, 3.0, 3.0),
        vec3::new(0.0, 0.0, 0.0),
        vec3::unit_y(),
        40.0,
        WIDTH as f32 / HEIGHT as f32,
        0.0,
        None,
    );

    let mut skybox = SkyBox::Color(Color::splat(0.0));

    match 5 {
        // Cornell Box
        0 => {
            let base = Color::splat(0.73);
            let left = Color::new(0.12, 0.45, 0.15);
            let right = Color::new(0.65, 0.05, 0.05);
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 555.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(left),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(0.0, 555.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(right),
                None,
            ));

            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(base),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 555.0, 0.0),
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(base),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 555.0),
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 555.0, 0.0),
                ),
                Lambertian::new(base),
                None,
            ));

            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(213.0, 554.0, 227.0),
                    vec3::new(130.0, 0.0, 0.0),
                    vec3::new(0.0, 0.0, 105.0),
                ),
                DiffuseLight::new(Color::splat(1.0), 15.0),
                None,
            ));

            objects.push(Object::new(
                hittables::AABB::new(vec3::splat(0.0), vec3::new(165.0, 330.0, 165.0)),
                Lambertian::new(base),
                Some(Transform::new(
                    mat4::translate(vec3::new(265.0, 0.0, 295.0))
                        * mat4::rotate(vec3::new(0.0, std::f32::consts::TAU / 24.0, 0.0)),
                )),
            ));

            objects.push(Object::new(
                hittables::AABB::new(vec3::splat(0.0), vec3::new(165.0, 165.0, 165.0)),
                Lambertian::new(base),
                Some(Transform::new(
                    mat4::translate(vec3::new(130.0, 0.0, 65.0))
                        * mat4::rotate(vec3::new(0.0, -std::f32::consts::TAU / 20.0, 0.0)),
                )),
            ));

            camera = Camera::new(
                maths::vec3::new(278.0, 278.0, -800.0),
                vec3::new(278.0, 278.0, 0.0),
                vec3::unit_y(),
                40.0,
                WIDTH as f32 / HEIGHT as f32,
                0.0,
                None,
            );
        }
        // orbs
        1 => {
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap(),
            );

            objects.push(Object::new(
                hittables::InfinitePlane::new(
                    vec3::new(0.0, -1.2, 0.0),
                    vec3::new(0.0, 1.0, 0.0).normalized(),
                ),
                Lambertian::new(Color::splat(0.3)),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(-1.0, 0.0, -1.0), 0.95),
                Metal::new(Color::new(0.8, 0.8, 0.8), 0.0),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(1.0, 0.0, -1.0), 0.95),
                Lambertian::new(Color::new(0.1, 0.1, 0.1)),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(1.0, 0.0, 1.0), 0.95),
                Metal::new(Color::new(0.3, 0.3, 0.3), 0.7),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), 0.95),
                Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)),
                None,
            ));
            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(-1.0, 0.0, 1.0), -0.85),
                Dielectric::new(1.2, Color::new(0.6, 0.9, 1.0)),
                None,
            ));
        }
        // glass suzanne
        2 => {
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap(),
            );

            objects.push(Object::new(
                hittables::InfinitePlane::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(0.0, 1.0, 0.0).normalized(),
                ),
                Lambertian::new(Color::splat(0.3)),
                None,
            ));

            let mut meshes = load_obj("assets/suzanne.obj").unwrap();
            let mesh = meshes.first_mut().unwrap();
            objects.push(Object::new(
                hittables::Mesh::new(&mesh.vertices, &mesh.indices),
                Dielectric::new(1.45, Color::splat(1.0)),
                Some(Transform::new(
                    mat4::rotate(vec3::new(0.0, 0.0, -35.0f32.to_radians()))
                        * mat4::translate(vec3::new(0.0, 0.6, 0.0)),
                )),
            ));
        }
        // cubes
        3 => {
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap(),
            );

            let n = 5;
            for x in 0..n {
                for y in 0..n {
                    for z in 0..n {
                        let p = vec3::new(
                            x as f32 - n as f32 / 2.0,
                            y as f32 - n as f32 / 2.0,
                            z as f32 - n as f32 / 2.0,
                        );
                        objects.push(Object::new(
                            hittables::AABB::new(vec3::splat(-0.35) + p, vec3::splat(0.35) + p),
                            Lambertian::new(
                                Color::new(
                                    x as f32 / n as f32,
                                    y as f32 / n as f32,
                                    z as f32 / n as f32,
                                ) * 0.8
                                    + Color::splat(0.1),
                            ),
                            Some(Transform::new(mat4::rotate(vec3::splat(
                                std::f32::consts::TAU / 8.0,
                            )))),
                        ));
                    }
                }
            }
        }
        // twisty boi
        4 => {
            camera = Camera::new(
                maths::vec3::new(0.0, 0.0, -5.0),
                vec3::splat(0.0),
                vec3::unit_y(),
                40.0,
                WIDTH as f32 / HEIGHT as f32,
                0.2,
                None,
            );

            objects.push(Object::new(
                hittables::InfinitePlane::new(
                    vec3::new(0.0, -1.5, 0.0),
                    vec3::new(0.0, 1.0, 0.0).normalized(),
                ),
                Lambertian::new(Color::splat(0.3)),
                None,
            ));

            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/snowy_forest_path_01_4k.hdr")).unwrap(),
            );

            objects.push(Object::new(
                hittables::HittableSDF::new(sdf::Torus::new(1.0, 0.5).twist(3.0)),
                Metal::new(Color::splat(0.4), 0.3),
                None,
            ));
        }
        // Cornell Box 2
        5 => {
            let base = Color::splat(0.73);
            let left = Color::new(0.12, 0.45, 0.15);
            let right = Color::new(0.65, 0.05, 0.05);
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 555.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(left),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(0.0, 555.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(right),
                None,
            ));

            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(base),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 555.0, 0.0),
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 0.0, 555.0),
                ),
                Lambertian::new(base),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 555.0),
                    vec3::new(555.0, 0.0, 0.0),
                    vec3::new(0.0, 555.0, 0.0),
                ),
                Lambertian::new(base),
                None,
            ));

            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(213.0, 554.0, 227.0),
                    vec3::new(130.0, 0.0, 0.0),
                    vec3::new(0.0, 0.0, 105.0),
                ),
                DiffuseLight::new(Color::splat(1.0), 15.0),
                None,
            ));

            // objects.push(Object::new(
            //     hittables::Sphere::new(vec3::new(278.0, 554.0, 279.5), 65.0),
            //     DiffuseLight::new(Color::splat(1.0), 15.0),
            //     None,
            // ));

            objects.push(Object::new(
                hittables::AABB::new(vec3::splat(0.0), vec3::new(165.0, 330.0, 165.0)),
                // Lambertian::new(base),
                Metal::new(Color::splat(0.85), 0.1),
                Some(Transform::new(
                    mat4::translate(vec3::new(265.0, 0.0, 295.0))
                        * mat4::rotate(vec3::new(0.0, std::f32::consts::TAU / 24.0, 0.0)),
                )),
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(190.0, 90.0, 190.0), 90.0),
                // DiffuseLight::new(Color::new(0.0, 0.0, 1.0), 0.1),
                Dielectric::new(1.5, Color::splat(1.0)),
                None,
            ));

            // objects.push(Object::new(
            //     hittables::Rect::new(
            //         vec3::splat(0.0),
            //         vec3::new(0.0, 319.04231694243947, 0.0),
            //         vec3::new(319.04231694243947, 0.0, 0.0),
            //     ),
            //     // DiffuseLight::new(Color::new(0.0, 0.0, 1.0), 0.1),
            //     Dielectric::new(1.5, Color::splat(1.0)),
            //     None,
            // ));

            camera = Camera::new(
                maths::vec3::new(278.0, 278.0, -800.0),
                vec3::new(278.0, 278.0, 0.0),
                vec3::unit_y(),
                35.0,
                WIDTH as f32 / HEIGHT as f32,
                0.0,
                None,
            );
        }
        _ => panic!("oof"),
    }

    let scene = Scene::new(camera, skybox, objects);

    Context::new(maths::vec2::new(WIDTH, HEIGHT), scene, ToneMap::HejlRichard)
}
