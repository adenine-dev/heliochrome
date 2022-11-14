use std::ops::*;

use super::{vec3, vec4};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct mat4 {
    cols: [vec4; 4],
}

impl mat4 {
    pub fn new(cols: [vec4; 4]) -> Self {
        Self { cols }
    }

    pub fn identity() -> Self {
        Self::new([
            vec4::new(1.0, 0.0, 0.0, 0.0),
            vec4::new(0.0, 1.0, 0.0, 0.0),
            vec4::new(0.0, 0.0, 1.0, 0.0),
            vec4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    pub fn translate(translate: vec3) -> Self {
        Self::new([
            vec4::new(1.0, 0.0, 0.0, 0.0),
            vec4::new(0.0, 1.0, 0.0, 0.0),
            vec4::new(0.0, 0.0, 1.0, 0.0),
            vec4::new(translate.x, translate.y, translate.z, 1.0),
        ])
    }

    pub fn scale(scale: vec3) -> Self {
        Self::new([
            vec4::new(scale.x, 0.0, 0.0, 0.0),
            vec4::new(0.0, scale.y, 0.0, 0.0),
            vec4::new(0.0, 0.0, scale.z, 0.0),
            vec4::new(0.0, 0.0, 0.0, 1.0),
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
            vec4::new(cos_a * cos_b, sin_a * cos_b, -sin_b, 0.0),
            vec4::new(
                cos_a * sin_b * sin_c - sin_a * cos_c,
                sin_a * sin_b * sin_c + cos_a * cos_c,
                cos_b * sin_c,
                0.0,
            ),
            vec4::new(
                cos_a * sin_b * cos_c + sin_a * sin_c,
                sin_a * sin_b * cos_c - cos_a * sin_c,
                cos_b * cos_c,
                0.0,
            ),
            vec4::new(0.0, 0.0, 0.0, 1.0),
        ])
    }

    pub fn rotate_deg(deg_euler_angles: vec3) -> Self {
        Self::rotate(deg_euler_angles.to_rad())
    }

    pub fn det(&self) -> f32 {
        let b00 = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        let b01 = self[0][0] * self[1][2] - self[0][2] * self[1][0];
        let b02 = self[0][0] * self[1][3] - self[0][3] * self[1][0];
        let b03 = self[0][1] * self[1][2] - self[0][2] * self[1][1];
        let b04 = self[0][1] * self[1][3] - self[0][3] * self[1][1];
        let b05 = self[0][2] * self[1][3] - self[0][3] * self[1][2];
        let b06 = self[2][0] * self[3][1] - self[2][1] * self[3][0];
        let b07 = self[2][0] * self[3][2] - self[2][2] * self[3][0];
        let b08 = self[2][0] * self[3][3] - self[2][3] * self[3][0];
        let b09 = self[2][1] * self[3][2] - self[2][2] * self[3][1];
        let b10 = self[2][1] * self[3][3] - self[2][3] * self[3][1];
        let b11 = self[2][2] * self[3][3] - self[2][3] * self[3][2];

        b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06
    }

    pub fn transposed(&self) -> Self {
        Self::new([
            vec4::new(
                self.cols[0].x,
                self.cols[1].x,
                self.cols[2].x,
                self.cols[3].x,
            ),
            vec4::new(
                self.cols[0].y,
                self.cols[1].y,
                self.cols[2].y,
                self.cols[3].y,
            ),
            vec4::new(
                self.cols[0].z,
                self.cols[1].z,
                self.cols[2].z,
                self.cols[3].z,
            ),
            vec4::new(
                self.cols[0].w,
                self.cols[1].w,
                self.cols[2].w,
                self.cols[3].w,
            ),
        ])
    }

    pub fn inverse(&self) -> Self {
        let b00 = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        let b01 = self[0][0] * self[1][2] - self[0][2] * self[1][0];
        let b02 = self[0][0] * self[1][3] - self[0][3] * self[1][0];
        let b03 = self[0][1] * self[1][2] - self[0][2] * self[1][1];
        let b04 = self[0][1] * self[1][3] - self[0][3] * self[1][1];
        let b05 = self[0][2] * self[1][3] - self[0][3] * self[1][2];
        let b06 = self[2][0] * self[3][1] - self[2][1] * self[3][0];
        let b07 = self[2][0] * self[3][2] - self[2][2] * self[3][0];
        let b08 = self[2][0] * self[3][3] - self[2][3] * self[3][0];
        let b09 = self[2][1] * self[3][2] - self[2][2] * self[3][1];
        let b10 = self[2][1] * self[3][3] - self[2][3] * self[3][1];
        let b11 = self[2][2] * self[3][3] - self[2][3] * self[3][2];

        let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

        mat4::new([
            vec4::new(
                (self[1][1] * b11 - self[1][2] * b10 + self[1][3] * b09) * inv_det,
                (self[0][2] * b10 - self[0][1] * b11 - self[0][3] * b09) * inv_det,
                (self[3][1] * b05 - self[3][2] * b04 + self[3][3] * b03) * inv_det,
                (self[2][2] * b04 - self[2][1] * b05 - self[2][3] * b03) * inv_det,
            ),
            vec4::new(
                (self[1][2] * b08 - self[1][0] * b11 - self[1][3] * b07) * inv_det,
                (self[0][0] * b11 - self[0][2] * b08 + self[0][3] * b07) * inv_det,
                (self[3][2] * b02 - self[3][0] * b05 - self[3][3] * b01) * inv_det,
                (self[2][0] * b05 - self[2][2] * b02 + self[2][3] * b01) * inv_det,
            ),
            vec4::new(
                (self[1][0] * b10 - self[1][1] * b08 + self[1][3] * b06) * inv_det,
                (self[0][1] * b08 - self[0][0] * b10 - self[0][3] * b06) * inv_det,
                (self[3][0] * b04 - self[3][1] * b02 + self[3][3] * b00) * inv_det,
                (self[2][1] * b02 - self[2][0] * b04 - self[2][3] * b00) * inv_det,
            ),
            vec4::new(
                (self[1][1] * b07 - self[1][0] * b09 - self[1][2] * b06) * inv_det,
                (self[0][0] * b09 - self[0][1] * b07 + self[0][2] * b06) * inv_det,
                (self[3][1] * b01 - self[3][0] * b03 - self[3][2] * b00) * inv_det,
                (self[2][0] * b03 - self[2][1] * b01 + self[2][2] * b00) * inv_det,
            ),
        ])
    }

    pub fn trans_mul(self, rhs: vec4) -> vec4 {
        #[rustfmt::skip] // lines long
        vec4::new(
            (self.cols[0].x * rhs.x) + (self.cols[0].y * rhs.y) + (self.cols[0].z * rhs.z) + (self.cols[0].w * rhs.w),
            (self.cols[1].x * rhs.x) + (self.cols[1].y * rhs.y) + (self.cols[1].z * rhs.z) + (self.cols[1].w * rhs.w),
            (self.cols[2].x * rhs.x) + (self.cols[2].y * rhs.y) + (self.cols[2].z * rhs.z) + (self.cols[2].w * rhs.w),
            (self.cols[3].x * rhs.x) + (self.cols[3].y * rhs.y) + (self.cols[3].z * rhs.z) + (self.cols[3].w * rhs.w),
        )
    }
}

impl Mul for mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        #[rustfmt::skip] // lines long
        Self::new([
            vec4::new(
                rhs[0][0]*self[0][0] + rhs[0][1]*self[1][0] + rhs[0][2]*self[2][0] + rhs[0][3]*self[3][0],
                rhs[0][0]*self[0][1] + rhs[0][1]*self[1][1] + rhs[0][2]*self[2][1] + rhs[0][3]*self[3][1],
                rhs[0][0]*self[0][2] + rhs[0][1]*self[1][2] + rhs[0][2]*self[2][2] + rhs[0][3]*self[3][2],
                rhs[0][0]*self[0][3] + rhs[0][1]*self[1][3] + rhs[0][2]*self[2][3] + rhs[0][3]*self[3][3],
            ),
            vec4::new(
                rhs[1][0]*self[0][0] + rhs[1][1]*self[1][0] + rhs[1][2]*self[2][0] + rhs[1][3]*self[3][0],
                rhs[1][0]*self[0][1] + rhs[1][1]*self[1][1] + rhs[1][2]*self[2][1] + rhs[1][3]*self[3][1],
                rhs[1][0]*self[0][2] + rhs[1][1]*self[1][2] + rhs[1][2]*self[2][2] + rhs[1][3]*self[3][2],
                rhs[1][0]*self[0][3] + rhs[1][1]*self[1][3] + rhs[1][2]*self[2][3] + rhs[1][3]*self[3][3],
            ),
            vec4::new(
                rhs[2][0]*self[0][0] + rhs[2][1]*self[1][0] + rhs[2][2]*self[2][0] + rhs[2][3]*self[3][0],
                rhs[2][0]*self[0][1] + rhs[2][1]*self[1][1] + rhs[2][2]*self[2][1] + rhs[2][3]*self[3][1],
                rhs[2][0]*self[0][2] + rhs[2][1]*self[1][2] + rhs[2][2]*self[2][2] + rhs[2][3]*self[3][2],
                rhs[2][0]*self[0][3] + rhs[2][1]*self[1][3] + rhs[2][2]*self[2][3] + rhs[2][3]*self[3][3],
            ),
            vec4::new(
                rhs[3][0]*self[0][0] + rhs[3][1]*self[1][0] + rhs[3][2]*self[2][0] + rhs[3][3]*self[3][0],
                rhs[3][0]*self[0][1] + rhs[3][1]*self[1][1] + rhs[3][2]*self[2][1] + rhs[3][3]*self[3][1],
                rhs[3][0]*self[0][2] + rhs[3][1]*self[1][2] + rhs[3][2]*self[2][2] + rhs[3][3]*self[3][2],
                rhs[3][0]*self[0][3] + rhs[3][1]*self[1][3] + rhs[3][2]*self[2][3] + rhs[3][3]*self[3][3],
            ),
        ])
    }
}

impl Mul<vec4> for mat4 {
    type Output = vec4;
    fn mul(self, rhs: vec4) -> vec4 {
        #[rustfmt::skip] // lines long
        vec4::new(
            (self.cols[0].x * rhs.x) + (self.cols[1].x * rhs.y) + (self.cols[2].x * rhs.z) + (self.cols[3].x * rhs.w),
            (self.cols[0].y * rhs.x) + (self.cols[1].y * rhs.y) + (self.cols[2].y * rhs.z) + (self.cols[3].y * rhs.w),
            (self.cols[0].z * rhs.x) + (self.cols[1].z * rhs.y) + (self.cols[2].z * rhs.z) + (self.cols[3].z * rhs.w),
            (self.cols[0].w * rhs.x) + (self.cols[1].w * rhs.y) + (self.cols[2].w * rhs.z) + (self.cols[3].w * rhs.w),
        )
    }
}

impl Div<f32> for mat4 {
    type Output = mat4;
    fn div(self, rhs: f32) -> mat4 {
        mat4::new([
            self.cols[0] / rhs,
            self.cols[1] / rhs,
            self.cols[2] / rhs,
            self.cols[3] / rhs,
        ])
    }
}

impl Index<usize> for mat4 {
    type Output = vec4;
    fn index(&self, index: usize) -> &Self::Output {
        &self.cols[index]
    }
}

impl IndexMut<usize> for mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cols[index]
    }
}
