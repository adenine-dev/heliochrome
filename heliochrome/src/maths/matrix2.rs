use std::ops::*;

use super::vec2;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct mat2 {
    cols: [vec2; 2],
}

impl mat2 {
    pub fn new(cols: [vec2; 2]) -> Self {
        Self { cols }
    }

    pub fn identity() -> Self {
        Self::new([vec2::new(1.0, 0.0), vec2::new(0.0, 1.0)])
    }

    pub fn scale(scale: vec2) -> Self {
        Self::new([vec2::new(scale.x, 0.0), vec2::new(0.0, scale.y)])
    }

    pub fn rotate(theta: f32) -> Self {
        let cos_a = theta.cos();
        let sin_a = theta.sin();
        Self::new([vec2::new(cos_a, -sin_a), vec2::new(sin_a, cos_a)])
    }

    pub fn det(&self) -> f32 {
        self.cols[0].x * self.cols[1].y - self.cols[0].y * self.cols[1].x
    }

    pub fn transposed(&self) -> Self {
        Self::new([
            vec2::new(self.cols[0].x, self.cols[1].x),
            vec2::new(self.cols[0].y, self.cols[1].y),
        ])
    }

    pub fn adulate(&self) -> Self {
        Self::new([
            vec2::new(self.cols[1].y, -self.cols[1].x),
            vec2::new(-self.cols[0].y, self.cols[0].x),
        ])
        .transposed()
    }

    pub fn inverse(&self) -> Self {
        self.adulate() / self.det()
    }
}

impl Mul for mat2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new([
            vec2::new(
                self[0].x * rhs[0].x + self[1].x * rhs[0].y,
                self[0].y * rhs[0].x + self[1].y * rhs[0].y,
            ),
            vec2::new(
                self[0].x * rhs[1].x + self[1].x * rhs[1].y,
                self[0].y * rhs[1].x + self[1].y * rhs[1].y,
            ),
        ])
    }
}

impl Mul<vec2> for mat2 {
    type Output = vec2;
    fn mul(self, rhs: vec2) -> vec2 {
        vec2::new(
            self[0].x * rhs.x + self[1].x * rhs.y,
            self[0].y * rhs.x + self[1].y * rhs.y,
        )
    }
}

impl Div<f32> for mat2 {
    type Output = mat2;
    fn div(self, rhs: f32) -> mat2 {
        mat2::new([self.cols[0] / rhs, self.cols[1] / rhs])
    }
}

impl Index<usize> for mat2 {
    type Output = vec2;
    fn index(&self, index: usize) -> &Self::Output {
        &self.cols[index]
    }
}

impl IndexMut<usize> for mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cols[index]
    }
}
