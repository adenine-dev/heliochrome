use crate::{
    accel::{Accel, Accelerator},
    camera::Camera,
    color::Color,
    hittables::Intersection,
    image::Image,
    materials::Scatterable,
    maths::{vec2, vec3, Ray},
    object::Object,
    pdf::ObjectPdf,
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
            SkyBox::Debug => ((dir.normalized() + vec3::splat(1.0)) * 0.5).into(),
        }
    }
}

pub struct Scene {
    pub camera: Camera,
    pub skybox: SkyBox,

    pub objects: Accel<Object>,
    pub important_indices: Vec<usize>,
}

impl Scene {
    pub fn new(camera: Camera, skybox: SkyBox, objects: Vec<Object>) -> Self {
        let objects = Accel::new(objects);
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

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Intersection, &Object)> {
        self.objects.intersect_obj(ray, t_min, t_max)
    }

    pub fn make_importance_pdf(&self, origin: &vec3) -> Vec<ObjectPdf> {
        self.important_indices
            .iter()
            .map(|idx| ObjectPdf::new(&self.objects.get_nth(*idx), *origin))
            .collect()
    }

    pub fn get_importants(&self) -> Vec<Object> {
        self.important_indices
            .iter()
            .map(|idx| self.objects.get_nth(*idx).clone())
            .collect()
    }
}
