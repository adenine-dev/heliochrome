use crate::{hittables::AABB, maths::vec3, sdf::SDF};

#[derive(Debug)]
pub struct MandelBulb {
    p: f32,
}

impl MandelBulb {
    pub fn new(p: f32) -> Self {
        Self { p }
    }
}

impl SDF for MandelBulb {
    fn dist(&self, p: vec3) -> f32 {
        const ITERATIONS: usize = 4;
        const BAILOUT: f32 = 256.0;

        let mut z = p;
        let mut dr = 1.0;
        let mut r = 0.0;
        for _ in 0..=ITERATIONS {
            r = z.mag();
            if r > BAILOUT {
                break;
            }

            let theta = (z.z / r).acos() * self.p;
            let stheta = theta.sin();
            let ctheta = theta.cos();
            let phi = z.y.atan2(z.x) * self.p;
            let sphi = phi.sin();
            let cphi = phi.cos();
            dr = r.powf(self.p - 1.0) * self.p * dr + 1.0;

            let zr = r.powf(self.p);

            z = zr * vec3::new(stheta * cphi, sphi * stheta, ctheta);
            z += p;
        }

        0.5 * r.ln() * r / dr
    }

    fn make_bounding_box(&self) -> AABB {
        AABB::new(vec3::splat(-2.0), vec3::splat(2.0))
    }
}
