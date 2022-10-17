use core::panic;

use rand::seq::SliceRandom;

use crate::{
    bvh::BVH,
    camera::Camera,
    color::Color,
    hittables::{Hit, Hittable},
    image::Image,
    materials::Scatterable,
    maths::{vec2, vec3, Ray},
    object::Object,
    pdf::{ObjectPDF, PDF},
};

pub enum SkyBox {
    Color(Color),
    Equirectangular(Image),
    Debug,
}

impl SkyBox {
    pub fn sample(&self, dir: vec3) -> Color {
        match self {
            SkyBox::Color(c) => *c,
            SkyBox::Equirectangular(img) => {
                let uv = vec2::new(
                    0.5 + dir.z.atan2(dir.x) / std::f32::consts::TAU,
                    0.5 + dir.y.asin() / std::f32::consts::PI,
                );

                img.sample_uv(&uv)
            }
            SkyBox::Debug => ((dir + vec3::splat(1.0)) / 2.0).into(),
        }
    }
}

pub struct Scene {
    pub camera: Camera,
    pub skybox: SkyBox,

    pub objects: BVH<Object>,
    pub important_indices: Vec<usize>,
}

impl Scene {
    pub fn new(camera: Camera, skybox: SkyBox, objects: Vec<Object>) -> Self {
        let objects = BVH::new(objects);
        let important_indices = objects
            .hittables
            .iter()
            .enumerate()
            .filter_map(|(idx, obj)| {
                if !obj.material.is_important() {
                    None
                } else {
                    Some(idx)
                }
            })
            .collect::<Vec<_>>();
        Self {
            camera,
            skybox,
            objects,
            important_indices,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, &Object)> {
        self.objects.hit_obj(ray, t_min, t_max)
    }

    pub fn make_importance_pdf(&self, origin: &vec3) -> Vec<ObjectPDF> {
        self.important_indices
            .iter()
            .map(|idx| ObjectPDF::new(self.objects.hittables[*idx].clone(), *origin))
            .collect()
    }

    pub fn get_importants(&self) -> Vec<Object> {
        self.important_indices
            .iter()
            .map(|idx| self.objects.hittables[*idx].clone())
            .collect()
    }
}
