use std::error::Error;

use crate::{
    loader::{parse_into, FromHCY},
    maths::*,
};

#[derive(Debug)]
pub struct Camera {
    pub eye: vec3,
    pub at: vec3,
    pub up: vec3,
    pub vfov: f32,
    pub size: vec2,
    pub aperture: f32,
    pub focus_dist: Option<f32>,
}

impl Camera {
    pub fn new(
        eye: vec3,
        at: vec3,
        up: vec3,
        vfov: f32,
        size: vec2,
        aperture: f32,
        focus_dist: Option<f32>,
    ) -> Self {
        Self {
            eye,
            at,
            up,
            vfov,
            size,
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
        let viewport_w = self.size.x / self.size.y * viewport_h;

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

impl FromHCY for Camera {
    //NOTE: you need to manually set size.
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut eye = None;
        let mut at = None;
        let mut up = None;
        let mut vfov = None;
        let mut aperture = None;
        let mut focus_dist = None;

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "eye" => eye = Some(parse_into(value)?),
                "at" => at = Some(parse_into(value)?),
                "up" => up = Some(parse_into(value)?),
                "vfov" => vfov = Some(parse_into(value)?),
                "aperture" => aperture = Some(parse_into(value)?),
                "focus_dist" => focus_dist = Some(parse_into(value)?),
                _ => {}
            }
        }

        Ok(Camera::new(
            eye.ok_or("could not find required key eye.")?,
            at.ok_or("could not find required key at.")?,
            up.ok_or("could not find required key up.")?,
            vfov.ok_or("could not find required key vfov.")?,
            vec2::splat(1.0),
            aperture.ok_or("could not find required key aperture.")?,
            focus_dist,
        ))
    }
}
