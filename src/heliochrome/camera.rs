use crate::maths::*;

pub struct Camera {
    pub eye: vec3,
    pub at: vec3,
    pub up: vec3,
    pub horizontal: vec3,
    pub vertical: vec3,
    pub lower_left: vec3,
    pub vfov: f32,
}

impl Camera {
    pub fn new(eye: vec3, at: vec3, up: vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let h = (vfov.to_radians() / 2.0).tan();
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = (eye - at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = viewport_w * u;
        let vertical = viewport_h * v;

        Self {
            eye,
            at,
            up,
            horizontal,
            vertical,
            lower_left: eye - horizontal / 2.0 - vertical / 2.0 - w,
            vfov,
        }
    }

    pub fn get_ray(&self, uv: &vec2) -> Ray {
        Ray::new(
            self.eye,
            (self.lower_left + uv.x * self.horizontal + uv.y * self.vertical - self.eye)
                .normalize(),
        )
    }
}
