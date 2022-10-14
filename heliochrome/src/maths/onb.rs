use super::vec3;

pub struct ONB {
    pub u: vec3,
    pub v: vec3,
    pub w: vec3,
}

impl ONB {
    pub fn new_from_w(w: vec3) -> Self {
        let a = if w.x.abs() > 0.9 {
            vec3::unit_y()
        } else {
            vec3::unit_x()
        };
        let v = a.cross(w).normalize();

        Self {
            u: v.cross(w),
            v,
            w,
        }
    }

    pub fn local(&self, a: &vec3) -> vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
