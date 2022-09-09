use std::ops::*;

use super::vec3;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct mat3 {
    cols: [vec3; 3],
}

impl mat3 {
    pub fn new(cols: [vec3; 3]) -> Self {
        Self { cols }
    }

    pub fn identity() -> Self {
        Self::new([
            vec3::new(1.0, 0.0, 0.0),
            vec3::new(0.0, 1.0, 0.0),
            vec3::new(0.0, 0.0, 1.0),
        ])
    }

    pub fn scale(scale: vec3) -> Self {
        Self::new([
            vec3::new(scale.x, 0.0, 0.0),
            vec3::new(0.0, scale.y, 0.0),
            vec3::new(0.0, 0.0, scale.z),
        ])
    }

    pub fn rotate(euler_angles: vec3) -> Self {
        let cos_a = euler_angles.x.cos();
        let sin_a = euler_angles.x.sin();
        let cos_b = euler_angles.y.cos();
        let sin_b = euler_angles.y.sin();
        let cos_c = euler_angles.z.cos();
        let sin_c = euler_angles.z.sin();
        Self::new([
            vec3::new(cos_a * cos_b, sin_a * cos_b, -sin_b),
            vec3::new(
                cos_a * sin_b * sin_c - sin_a * cos_c,
                sin_a * sin_b * sin_c + cos_a * cos_c,
                cos_a * sin_b,
            ),
            vec3::new(
                cos_a * sin_b * cos_c - sin_a * sin_c,
                cos_a * sin_b * sin_c + sin_a * cos_c,
                cos_a * cos_b,
            ),
        ])
    }

    pub fn det(&self) -> f32 {
        let a = self.cols[0].x;
        let b = self.cols[1].x;
        let c = self.cols[2].x;
        let d = self.cols[0].y;
        let e = self.cols[1].y;
        let f = self.cols[2].y;
        let g = self.cols[0].z;
        let h = self.cols[1].z;
        let i = self.cols[2].z;

        a * e * i + b * f * g + c * d * h - c * e * g - b * d * i - a * f * h
    }

    pub fn transposed(&self) -> Self {
        Self::new([
            vec3::new(self.cols[0].x, self.cols[1].x, self.cols[2].x),
            vec3::new(self.cols[0].y, self.cols[1].y, self.cols[2].y),
            vec3::new(self.cols[0].z, self.cols[1].z, self.cols[2].z),
        ])
    }

    pub fn adulate(&self) -> Self {
        let x = self.cols[1].cross(self.cols[2]);
        let y = self.cols[2].cross(self.cols[0]);
        let z = self.cols[0].cross(self.cols[1]);

        Self::new([x, y, z]).transposed()
    }

    pub fn inverse(&self) -> Self {
        self.adulate() / self.det()
    }
}

impl Mul for mat3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        #[rustfmt::skip] // lines long
        Self::new([
            vec3::new(
                (self.cols[0].x * rhs.cols[0].x) + (self.cols[1].x * rhs.cols[0].y) + (self.cols[2].x * rhs.cols[0].z),
                (self.cols[0].y * rhs.cols[0].x) + (self.cols[1].y * rhs.cols[0].y) + (self.cols[2].y * rhs.cols[0].z),
                (self.cols[0].z * rhs.cols[0].x) + (self.cols[1].z * rhs.cols[0].y) + (self.cols[2].z * rhs.cols[0].z),
            ),
            vec3::new(
                (self.cols[0].x * rhs.cols[1].x) + (self.cols[1].x * rhs.cols[1].y) + (self.cols[2].x * rhs.cols[1].z),
                (self.cols[0].y * rhs.cols[1].x) + (self.cols[1].y * rhs.cols[1].y) + (self.cols[2].y * rhs.cols[1].z),
                (self.cols[0].z * rhs.cols[1].x) + (self.cols[1].z * rhs.cols[1].y) + (self.cols[2].z * rhs.cols[1].z),
            ),
            vec3::new(
                (self.cols[0].x * rhs.cols[2].x) + (self.cols[1].x * rhs.cols[2].y) + (self.cols[2].x * rhs.cols[2].z),
                (self.cols[0].y * rhs.cols[2].x) + (self.cols[1].y * rhs.cols[2].y) + (self.cols[2].y * rhs.cols[2].z),
                (self.cols[0].z * rhs.cols[2].x) + (self.cols[1].z * rhs.cols[2].y) + (self.cols[2].z * rhs.cols[2].z),
            ),
        ])
    }
}

impl Mul<vec3> for mat3 {
    type Output = vec3;
    fn mul(self, rhs: vec3) -> vec3 {
        vec3::new(
            (self.cols[0].x * rhs.x) + (self.cols[1].x * rhs.y) + (self.cols[2].x * rhs.z),
            (self.cols[0].y * rhs.x) + (self.cols[1].y * rhs.y) + (self.cols[2].y * rhs.z),
            (self.cols[0].z * rhs.x) + (self.cols[1].z * rhs.y) + (self.cols[2].z * rhs.z),
        )
    }
}

impl Div<f32> for mat3 {
    type Output = mat3;
    fn div(self, rhs: f32) -> mat3 {
        mat3::new([self.cols[0] / rhs, self.cols[1] / rhs, self.cols[2] / rhs])
    }
}

impl Index<usize> for mat3 {
    type Output = vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.cols[index]
    }
}

impl IndexMut<usize> for mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cols[index]
    }
}
