use super::Scatter;
use crate::{color::Color, hittables::Hit, materials::Scatterable, maths::Ray, pdf::CosinePDF};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        // let uvw = ONB::new_from_w(hit.normal);
        // let dir = uvw.local(&vec3::random_cosine_direction());
        Some(Scatter {
            // outgoing: Ray::new(hit.p, dir),
            attenuation: self.albedo,
            pdf: Some(CosinePDF::new(hit.normal).into()),
            specular: None,
        })
    }

    fn pdf(&self, _incoming: &Ray, outgoing: &Ray, hit: &Hit) -> f32 {
        let cosine = hit.normal.dot(outgoing.direction.normalized());
        if cosine > 0.0 {
            cosine / std::f32::consts::PI
        } else {
            0.0
        }
    }
}
