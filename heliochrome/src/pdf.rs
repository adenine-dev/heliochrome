use enum_dispatch::enum_dispatch;

use crate::{
    hittables::Hittable,
    maths::{vec3, ONB},
    object::Object,
};

#[enum_dispatch]
pub trait ProbabilityDensityFn {
    fn value(&self, dir: &vec3) -> f32;
    fn generate(&self) -> vec3;
}

pub struct CosinePDF {
    onb: ONB,
}

impl CosinePDF {
    pub fn new(w: vec3) -> Self {
        Self {
            onb: ONB::new_from_w(w),
        }
    }
}

impl ProbabilityDensityFn for CosinePDF {
    fn value(&self, dir: &vec3) -> f32 {
        let cosine = dir.normalized().dot(self.onb.w);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f32::consts::PI
        }
    }

    fn generate(&self) -> vec3 {
        self.onb.local(&vec3::random_cosine_direction())
    }
}

pub struct ObjectPDF {
    obj: Object,
    origin: vec3,
}

impl ObjectPDF {
    pub fn new(obj: Object, origin: vec3) -> Self {
        Self { obj, origin }
    }
}

impl ProbabilityDensityFn for ObjectPDF {
    fn value(&self, dir: &vec3) -> f32 {
        self.obj.pdf_value(&self.origin, dir)
    }

    fn generate(&self) -> vec3 {
        (self.obj.random_point_on() - self.origin).normalize()
    }
}

impl<A: ProbabilityDensityFn, B: ProbabilityDensityFn> ProbabilityDensityFn for (A, B) {
    fn value(&self, dir: &vec3) -> f32 {
        self.0.value(dir) * 0.5 + self.1.value(dir) * 0.5
    }

    fn generate(&self) -> vec3 {
        if rand::random::<f32>() < 0.5 {
            self.0.generate()
        } else {
            self.1.generate()
        }
    }
}

#[enum_dispatch(ProbabilityDensityFn)]
pub enum PDF {
    CosinePDF,
    ObjectPDF,
}
