use crate::{
    bvh::BVH,
    camera::Camera,
    color::Color,
    hittables::Hit,
    image::Image,
    maths::{vec2, vec3, Ray},
    object::Object,
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
}

impl Scene {
    pub fn new(camera: Camera, skybox: SkyBox, objects: Vec<Object>) -> Self {
        Self {
            camera,
            skybox,
            objects: BVH::new(objects),
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, &Object)> {
        self.objects.hit_obj(ray, t_min, t_max)
    }
}
