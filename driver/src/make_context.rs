// const WIDTH: f32 = 1280.0;
// const HEIGHT: f32 = 720.0;
// const WIDTH: f32 = 800.0;
// const HEIGHT: f32 = 345.0;
// const WIDTH: f32 = 300.0;
// const HEIGHT: f32 = 300.0;

use std::{fs, path::Path};

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
    let mut width = 300.0;
    let mut height = 300.0;

    let mut objects = vec![];
    let mut camera = Camera::new(
        maths::vec3::new(3.0, 3.0, 3.0),
        vec3::new(0.0, 0.0, 0.0),
        vec3::unit_y(),
        40.0,
        width as f32 / height as f32,
        0.0,
        None,
    );
    let mut tone_map = ToneMap::HejlRichard;
    let mut skybox = SkyBox::Color(Color::splat(0.0));

    match 8 {
        // Cornell Box
        0 => {
            width = 400.0;
            height = 400.0;

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
                width as f32 / height as f32,
                0.0,
                None,
            );
        }
        // orbs
        1 => {
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/skyboxes/snowy_forest_path_01_4k.hdr"))
                    .unwrap(),
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
                width as f32 / height as f32,
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
            skybox = SkyBox::Color(Color::splat(0.0));
            let s = 2.0 / 4.0;
            width = 750.0;
            height = width * s;

            let w = 555.0;
            let h = 555.0 * s;
            let d = 555.0;

            let base = Color::splat(0.73);
            let left = Color::new(0.12, 0.12, 0.75);
            let right = Color::new(0.65, 0.05, 0.05);
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(w, 0.0, 0.0),
                    vec3::new(0.0, h, 0.0),
                    vec3::new(0.0, 0.0, d),
                ),
                Lambertian::new(left),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(0.0, h, 0.0),
                    vec3::new(0.0, 0.0, d),
                ),
                Lambertian::new(right),
                None,
            ));

            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, 0.0),
                    vec3::new(w, 0.0, 0.0),
                    vec3::new(0.0, 0.0, d),
                ),
                Lambertian::new(base),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, h, 0.0),
                    vec3::new(w, 0.0, 0.0),
                    vec3::new(0.0, 0.0, d),
                ),
                Lambertian::new(base),
                None,
            ));
            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(0.0, 0.0, d),
                    vec3::new(w, 0.0, 0.0),
                    vec3::new(0.0, h, 0.0),
                ),
                Lambertian::new(base),
                None,
            ));

            objects.push(Object::new(
                hittables::Rect::new(
                    vec3::new(213.0, h - 1.0, 227.0),
                    vec3::new(130.0, 0.0, 0.0),
                    vec3::new(0.0, 0.0, 105.0),
                ),
                DiffuseLight::new(Color::splat(1.0), 15.0),
                None,
            ));

            let meshes = load_obj("assets/models/smoothanne.obj").unwrap();
            let mesh = meshes.first().unwrap();
            objects.push(Object::new(
                hittables::Mesh::new(&mesh.vertices, &mesh.indices, &mesh.normals),
                Lambertian::new(base),
                Some(Transform::new(
                    mat4::translate(vec3::new(277.5, 50.0, 277.5))
                        * mat4::rotate(vec3::new(
                            0.0,
                            std::f32::consts::PI + std::f32::consts::TAU / 12.0,
                            -35.0f32.to_radians(),
                        ))
                        * mat4::scale(vec3::splat(100.0)),
                )),
            ));

            camera = Camera::new(
                maths::vec3::new(w / 2.0, h / 2.0, -1000.0),
                vec3::new(w / 2.0, h / 2.0, 0.0),
                vec3::unit_y(),
                15.0,
                width as f32 / height as f32,
                0.0,
                None,
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
                width as f32 / height as f32,
                0.0,
                None,
            );

            tone_map = ToneMap::Reinhard(1.0);
        }
        // materials
        7 => {
            width = 500.0;
            height = 200.0;
            camera = Camera::new(
                maths::vec3::new(0.0, 1.0, 18.0),
                vec3::new(0.0, 0.0, 0.0),
                vec3::unit_y(),
                40.0,
                width as f32 / height as f32,
                0.1,
                None,
            );
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new("assets/skyboxes/belfast_sunset_puresky_4k.hdr"))
                    .unwrap(),
            );

            // skybox = SkyBox::Color(Color::splat(0.85));

            let files = fs::read_dir("assets/bsdfs")
                .unwrap()
                .flatten()
                .collect::<Vec<_>>();
            let cols = 10;
            for (i, path) in files.iter().enumerate() {
                let r = i % (cols);
                let c = i / cols;
                let s = 2.2;
                let x = r as f32 - cols as f32 / 2.0 + 0.5;
                let y = c as f32;
                let z = 0.0;
                objects.push(Object::new(
                    hittables::Sphere::new(vec3::new(x, y, z) * s, 1.0),
                    Measured::load_from_rgl_rgb_bsdf(path.path().as_path()).unwrap(),
                    None,
                ));
            }

            // objects.push(Object::new(
            //     hittables::Sphere::new(vec3::splat(0.0), 1.0),
            //     // BRDF::load_from_rgl_rgb_bsdf(Path::new("assets/bsdfs/acrylic_felt_pink_rgb.bsdf"))
            //     //     .unwrap(),
            //     BRDF::load_from_rgl_rgb_bsdf(Path::new(
            //         "assets/bsdfs/aniso_morpho_melenaus_rgb.bsdf",
            //     ))
            //     .unwrap(),
            //     // BRDF::load_from_rgl_rgb_bsdf(Path::new("assets/bsdfs/chm_mint_rgb.bsdf")).unwrap(),
            //     None,
            // ));

            objects.push(Object::new(
                hittables::InfinitePlane::new(
                    vec3::new(0.0, -1.2, 0.0),
                    vec3::new(0.0, 1.0, 0.0).normalized(),
                ),
                Metal::new(Color::splat(0.25), 0.2),
                // Lambertian::new(Color::splat(0.85)),
                None,
            ));

            tone_map = ToneMap::Clamp;
        }
        8 => {
            width = 400.0;
            height = 400.0;
            camera = Camera::new(
                maths::vec3::new(110.0, 110.0, 650.0),
                vec3::new(110.0, 110.0, 0.0),
                vec3::unit_y(),
                40.0,
                width as f32 / height as f32,
                0.0,
                None,
            );
            skybox = SkyBox::Equirectangular(
                Image::load_from_hdri(Path::new(
                    "assets/skyboxes/christmas_photo_studio_05_4k.hdr",
                ))
                .unwrap(),
            );

            // skybox = SkyBox::Color(Color::splat(0.85));
            // tone_map = ToneMap::Clamp;

            objects.push(Object::new(
                hittables::Sphere::new(vec3::splat(0.0), 100.0),
                Measured::load_from_rgl_rgb_bsdf(Path::new(
                    "assets/bsdfs/acrylic_felt_pink_rgb.bsdf",
                ))
                .unwrap(),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(220.0, 0.0, 0.0), 100.0),
                Measured::load_from_rgl_rgb_bsdf(Path::new(
                    "assets/bsdfs/aniso_morpho_melenaus_rgb.bsdf",
                ))
                .unwrap(),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(0.0, 220.0, 0.0), 100.0),
                Measured::load_from_rgl_rgb_bsdf(Path::new("assets/bsdfs/cg_sunflower_rgb.bsdf"))
                    .unwrap(),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(220.0, 220.0, 0.0), 100.0),
                Measured::load_from_rgl_rgb_bsdf(Path::new("assets/bsdfs/chm_mint_rgb.bsdf"))
                    .unwrap(),
                None,
            ));

            // let meshes = load_obj("assets/models/ornament.obj").unwrap();
            // dbg!(meshes.len());
            // let mesh = meshes.first().unwrap();
            // objects.push(Object::new(
            //     hittables::Mesh::new(&mesh.vertices, &mesh.indices, &mesh.normals),
            //     Metal::new(Color::splat(0.95), 0.0),
            //     Some(Transform::new(
            //         mat4::scale(vec3::splat(0.3)) * mat4::translate(vec3::new(0.0, 5.0, 0.0)),
            //     )),
            // ));

            objects.push(Object::new(
                hittables::InfinitePlane::new(
                    vec3::new(0.0, -110.0, 0.0),
                    vec3::new(0.0, 1.0, 0.0).normalized(),
                ),
                Metal::new(Color::splat(0.25), 0.2),
                // Lambertian::new(Color::splat(0.85)),
                None,
            ));

            objects.push(Object::new(
                hittables::Sphere::new(vec3::new(110.0, 110.0, 1000.0), 100.0),
                DiffuseLight::new(Color::splat(1.0), 20.0),
                None,
            ));
        }
        _ => panic!("oof"),
    }

    let scene = Scene::new(camera, skybox, objects);

    Context::new(maths::vec2::new(width, height), scene, tone_map)
}
