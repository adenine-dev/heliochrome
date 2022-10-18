use rand::random;

use crate::{
    color::Color,
    hittables::Hit,
    materials::{Scatter, Scatterable},
    maths::Ray,
};

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f32,
    pub color: Color,
}

impl Dielectric {
    pub fn new(ir: f32, color: Color) -> Self {
        Self { ir, color }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_dir = ray.direction.normalized();
        let cos_theta = (-unit_dir).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_reflect = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_reflect || reflectance(cos_theta, refraction_ratio) > random() {
            unit_dir.reflect_over(hit.normal)
        } else {
            unit_dir.refract(hit.normal, refraction_ratio)
        };

        Some(Scatter {
            attenuation: self.color,
            pdf: None,
            specular: Some(Ray::new(hit.p, direction)),
        })
    }

    fn is_important(&self) -> bool {
        false
    }
}
