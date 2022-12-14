use std::path::Path;

use heliochrome::{
    camera::Camera,
    color::Color,
    context::{Context, QualitySettings},
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
    let mut width = 500.0;
    let mut height = 500.0;

    let mut objects = vec![];
    let mut camera = Camera::new(
        vec3::new(3.0, 3.0, 3.0),
        vec3::new(0.0, 0.0, 0.0),
        vec3::unit_y(),
        40.0,
        vec2::new(width, height),
        0.0,
        None,
    );
    let mut tone_map = ToneMap::HejlRichard;
    let mut skybox = SkyBox::Color(Color::splat(0.0));

    match 5 {
        // Cornell Box
        0 => {
            return Context::new_from_config(
                loader::load_scene_file(Path::new("assets/scene/cornell_box.hcy")).unwrap(),
            );
        }
        // orbs
        1 => {
            return Context::new_from_config(
                loader::load_scene_file(Path::new("assets/scene/orbs.hcy")).unwrap(),
            );
        }
        // glass suzanne
        2 => {
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/skyboxes/snowy_forest_path_01_4k.hdr"))
                    .unwrap(),
            );

            objects.push(Object::new(
                hittables::InfinitePlane::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(0.0, 1.0, 0.0).normalized(),
                ),
                Lambertian::new(Color::splat(0.3)),
                None,
            ));

            let mut meshes = load_obj("assets/models/suzanne.obj").unwrap();
            let mesh = meshes.first_mut().unwrap();
            objects.push(Object::new(
                hittables::Mesh::new(&mesh.vertices, &mesh.indices, &[]),
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
                Image::load_from_hdri(Path::new("assets/skyboxes/snowy_forest_path_01_4k.hdr"))
                    .unwrap(),
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
                vec2::new(width, height),
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
                Image::load_from_hdri(Path::new("assets/skyboxes/snowy_forest_path_01_4k.hdr"))
                    .unwrap(),
            );

            objects.push(Object::new(
                hittables::HittableSDF::new(sdf::Torus::new(1.0, 0.5).twist(3.0)),
                Dielectric::new(1.45, Color::splat(1.0)),
                None,
            ));
        }
        // Box with suzanne
        5 => {
            return Context::new_from_config(
                loader::load_scene_file(Path::new("assets/scene/cornell_monkey.hcy")).unwrap(),
            );
        }
        // mandel bulb
        6 => {
            width = 200.0;
            height = 200.0;
            skybox = SkyBox::Color(Color::splat(0.05));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(0.0, 5.0, 0.0), 2.0),
                DiffuseLight::new(Color::new(1.0, 0.1, 0.2), 5.0),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(0.0, -5.0, 0.0), 2.0),
                DiffuseLight::new(Color::new(0.2, 0.1, 1.0), 5.0),
                None,
            ));

            objects.push(Object::new(
                hittables::HittableSDF::new(sdf::MandelBulb::new(8.0)),
                Lambertian::new(Color::new(0.85, 0.75, 0.85)),
                None,
            ));

            // objects.push(Object::new(
            //     hittables::Sphere::new(vec3::default(), 1.0),
            //     Lambertian::new(Color::new(0.85, 0.85, 0.85)),
            //     None,
            // ));

            camera = Camera::new(
                maths::vec3::new(-3.0, 0.0, -3.0),
                vec3::splat(0.0),
                vec3::unit_y(),
                35.0,
                vec2::new(width, height),
                0.0,
                None,
            );

            tone_map = ToneMap::Reinhard(1.0);
        }
        // bit of everything
        7 => {
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new(
                    "assets/skyboxes/christmas_photo_studio_05_4k.hdr",
                ))
                .unwrap(),
            );
            camera.eye = vec3::new(3.0, 3.0, -26.0);

            objects.push(Object::new(
                hittables::InfinitePlane::new(
                    vec3::new(0.0, -1.2, 0.0),
                    vec3::new(0.0, 1.0, 0.0).normalized(),
                ),
                Lambertian::new(Color::splat(0.3)),
                None,
            ));

            objects.push(Object::new(
                hittables::HittableSDF::new(sdf::Torus::new(1.0, 0.5)),
                Metal::new(Color::splat(0.85), 0.0),
                None,
            ));

            objects.push(Object::new(
                hittables::AABB::new(vec3::splat(-1.0), vec3::splat(1.0)),
                Lambertian::new(Color::new(0.85, 0.2, 0.2)),
                Some(Transform::new(
                    mat4::translate(vec3::new(3.0, 0.0, 0.0))
                        * mat4::rotate(vec3::splat(std::f32::consts::TAU / 8.0)),
                )),
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(0.0, 5.0, 0.0), 1.5),
                Dielectric::new(1.45, Color::new(0.7, 0.7, 0.2)),
                Some(Transform::new(mat4::scale(vec3::new(2.0, 1.0, 1.0)))),
            ));

            let meshes = load_obj("assets/models/suzanne.obj").unwrap();
            let mesh = meshes.first().unwrap();

            objects.push(Object::new(
                hittables::Mesh::new(&mesh.vertices, &mesh.indices, &mesh.normals),
                Metal::new(Color::splat(0.5), 0.5),
                Some(Transform::new(
                    mat4::rotate(vec3::new(0.0, std::f32::consts::TAU / 2.0, 0.0))
                        * mat4::translate(vec3::new(2.0, 1.0, 1.0)),
                )),
            ));

            objects.push(Object::new(
                hittables::Triangle::new([
                    vec3::new(-7.0, 0.0, -5.0),
                    vec3::new(-7.0, 10.0, 0.0),
                    vec3::new(-7.0, 0.0, 5.0),
                ]),
                Lambertian::new(Color::new(0.2, 0.7, 0.2)),
                None,
            ));

            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(7.0, 0.0, -5.0),
                    vec3::new(0.0, 10.0, 0.0),
                    vec3::new(0.0, 0.0, 10.0),
                ),
                Lambertian::new(Color::new(0.85, 0.2, 0.2)),
                None,
            ));
        }
        // many orbs
        8 => {
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/skyboxes/snowy_forest_path_01_4k.hdr"))
                    .unwrap(),
            );

            let s = 20i32;
            camera.eye = vec3::splat(s as f32);
            for y in 0..s {
                for x in 0..s {
                    let material: Material = match (x + y) % 3 {
                        0 => Metal::new(Color::new(0.8, 0.8, 0.8), 0.0).into(),
                        1 => Lambertian::new(Color::new(0.8, 0.8, 0.8)).into(),
                        2 => Dielectric::new(1.5, Color::splat(1.0)).into(),
                        _ => panic!(),
                    };
                    objects.push(Object::new(
                        hittables::Sphere::new(
                            vec3::new((x - s / 2) as f32, (y - s / 2) as f32, 0.0),
                            0.45,
                        ),
                        material,
                        None,
                    ));
                }
            }
        }
        _ => panic!("oof"),
    }

    let scene = Scene::new(camera, skybox, objects);

    Context::new(
        maths::vec2::new(width, height),
        QualitySettings::default(),
        scene,
        tone_map,
    )
}
