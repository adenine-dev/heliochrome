use crate::maths::*;

pub struct Camera {
    pub eye: vec3,
    pub horizontal: vec3,
    pub vertical: vec3,
    pub lower_left: vec3,
}

impl Camera {
    pub fn new(eye: vec3, aspect_ratio: f32) -> Self {
        let viewport_h = 2.0;
        let viewport_w = aspect_ratio * viewport_h;
        let horizontal = vec3::new(viewport_w, 0.0, 0.0);
        let vertical = vec3::new(0.0, viewport_h, 0.0);

        Self {
            eye,
            horizontal,
            vertical,
            lower_left: eye - horizontal / 2.0 - vertical / 2.0 - vec3::new(0.0, 0.0, 1.0),
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
