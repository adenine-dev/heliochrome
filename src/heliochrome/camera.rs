use crate::maths::*;

pub struct Camera {
    pub eye: vec3,
    pub at: vec3,
    pub up: vec3,
    pub vfov: f32,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(eye: vec3, at: vec3, up: vec3, vfov: f32, aspect_ratio: f32) -> Self {
        Self {
            eye,
            at,
            up,
            vfov,
            aspect_ratio,
        }
    }

    pub fn get_ray(&self, uv: &vec2) -> Ray {
        let h = (self.vfov.to_radians() / 2.0).tan();
        let viewport_h = 2.0 * h;
        let viewport_w = self.aspect_ratio * viewport_h;

        let w = (self.eye - self.at).normalize();
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = viewport_w * u;
        let vertical = viewport_h * v;

        let lower_left = self.eye - horizontal / 2.0 - vertical / 2.0 - w;

        Ray::new(
            self.eye,
            (lower_left + uv.x * horizontal + uv.y * vertical - self.eye).normalize(),
        )
    }
}
