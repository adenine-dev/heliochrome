use crate::maths::*;

pub struct Camera {
    pub eye: vec3,
    pub at: vec3,
    pub up: vec3,
    pub vfov: f32,
    pub aspect_ratio: f32,
    pub aperture: f32,
    pub focus_dist: Option<f32>,
}

impl Camera {
    pub fn new(
        eye: vec3,
        at: vec3,
        up: vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: Option<f32>,
    ) -> Self {
        Self {
            eye,
            at,
            up,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
        }
    }

    pub fn get_default_focus_dist(&self) -> f32 {
        (self.eye - self.at).mag()
    }

    pub fn get_ray(&self, uv: &vec2) -> Ray {
        let h = (self.vfov.to_radians() * 0.5).tan();
        let viewport_h = 2.0 * h;
        let viewport_w = self.aspect_ratio * viewport_h;

        let w = (self.eye - self.at).normalize();
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        let focus_dist = self
            .focus_dist
            .unwrap_or_else(|| self.get_default_focus_dist());

        let horizontal = focus_dist * viewport_w * u;
        let vertical = focus_dist * viewport_h * v;

        let lower_left = self.eye - horizontal * 0.5 - vertical * 0.5 - focus_dist * w;

        let rd = (self.aperture * 0.5) * vec3::random_in_unit_xy_disk();
        let offset = u * rd.x + v * rd.y;

        Ray::new(
            self.eye + offset,
            (lower_left + uv.x * horizontal + uv.y * vertical - self.eye - offset).normalize(),
        )
    }
}
