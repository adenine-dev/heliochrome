use crate::maths::vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: vec3,
    pub direction: vec3,
}

impl Ray {
    pub fn new(origin: vec3, direction: vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> vec3 {
        self.origin + t * self.direction
    }
}
