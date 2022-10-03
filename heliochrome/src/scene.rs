use crate::{
    bvh::BVH,
    camera::Camera,
    color::Color,
    hittables::{Hit, Hittable},
    image::Image,
    maths::{vec2, vec3, Ray},
    object::Object,
};

pub struct ObjectStore {
    bounded_objects: BVH<Object>,
    unbounded_objects: Vec<Object>,
}

impl ObjectStore {
    pub fn new(objects: Vec<Object>) -> ObjectStore {
        let (bounded_objects, unbounded_objects) = BVH::new(objects);

        ObjectStore {
            bounded_objects,
            unbounded_objects,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<(Hit, &Object)> {
        let mut res: Option<(Hit, &Object)> = None;

        for object in self.unbounded_objects.iter() {
            let hit = object.hit(ray, t_min, t_max);
            if let Some(hit) = hit {
                t_max = hit.t;
                res = Some((hit, object));
            }
        }

        let o = self.bounded_objects.hit_obj(ray, t_min, t_max);
        if o.is_some() && (res.is_none() || o.as_ref().unwrap().0.t < res.as_ref().unwrap().0.t) {
            res = o
        }

        res
    }
}

pub enum SkyBox {
    Color(Color),
    Equirectangular(Image),
    Debug,
}

impl SkyBox {
    pub fn sample(&self, dir: vec3) -> Color {
        match self {
            SkyBox::Color(c) => *c,
            SkyBox::Equirectangular(i) => {
                let uv = vec2::new(
                    0.5 + dir.z.atan2(dir.x) / std::f32::consts::TAU,
                    0.5 + dir.y.asin() / std::f32::consts::PI,
                );
                i.sample_uv(&uv)
            }
            SkyBox::Debug => ((dir + vec3::splat(1.0)) / 2.0).into(),
        }
    }
}

pub struct Scene {
    pub camera: Camera,
    pub skybox: SkyBox,

    pub objects: ObjectStore,
}

impl Scene {
    pub fn new(camera: Camera, skybox: SkyBox, objects: Vec<Object>) -> Self {
        Self {
            camera,
            skybox,
            objects: ObjectStore::new(objects),
        }
    }
}
