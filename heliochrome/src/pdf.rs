use enum_dispatch::enum_dispatch;
use rand::seq::SliceRandom;

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
pub struct CosinePdf {
    onb: ONB,
}

impl CosinePdf {
    pub fn new(w: vec3) -> Self {
        Self {
            onb: ONB::new_from_w(w),
        }
    }
}

impl ProbabilityDensityFn for CosinePdf {
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
pub struct ObjectPdf<'a> {
    pub obj: &'a Object,
    pub origin: vec3,
}

impl<'a> ObjectPdf<'a> {
    pub fn new(obj: &'a Object, origin: vec3) -> Self {
        Self { obj, origin }
    }
}

impl<'a> ProbabilityDensityFn for ObjectPdf<'a> {
    fn value(&self, dir: &vec3) -> f32 {
        self.obj.pdf_value(&self.origin, dir)
    }

    fn generate(&self) -> vec3 {
        self.obj.random(&self.origin)
    }
}

pub struct ObjectListPdf {
    pub objs: Vec<Object>,
    pub origin: vec3,
}

impl ObjectListPdf {
    pub fn new(objs: Vec<Object>, origin: vec3) -> Self {
        Self { objs, origin }
    }
}

impl ProbabilityDensityFn for ObjectListPdf {
    fn value(&self, dir: &vec3) -> f32 {
        self.objs
            .iter()
            .fold(0.0, |sum, obj| sum + obj.pdf_value(&self.origin, dir))
            / self.objs.len() as f32
    }

    fn generate(&self) -> vec3 {
        self.objs
            .choose(&mut rand::thread_rng())
            .unwrap()
            .random(&self.origin)
    }
}

#[enum_dispatch(ProbabilityDensityFn)]
pub enum Pdf<'a> {
    CosinePdf,
    ObjectPdf(ObjectPdf<'a>),
    ObjectListPdf,
}

impl<P: ProbabilityDensityFn> ProbabilityDensityFn for Vec<P> {
    fn value(&self, dir: &vec3) -> f32 {
        self.iter().fold(0.0, |sum, pdf| sum + pdf.value(dir)) / self.len() as f32
    }

    fn generate(&self) -> vec3 {
        self.choose(&mut rand::thread_rng()).unwrap().generate()
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
