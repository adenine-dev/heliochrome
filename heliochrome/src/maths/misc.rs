use std::simd::{LaneCount, Simd, SimdFloat, SupportedLaneCount};

#[const_trait]
pub trait Splat<T> {
    fn splat(val: T) -> Self;
}

impl const Splat<f32> for f32 {
    fn splat(val: f32) -> Self {
        val
    }
}

impl const Splat<f64> for f64 {
    fn splat(val: f64) -> Self {
        val
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub trait FloatOps {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl<const LANES: usize> FloatOps for Simd<f32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    fn min(self, other: Self) -> Self {
        self.simd_min(other)
    }

    fn max(self, other: Self) -> Self {
        self.simd_max(other)
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.simd_clamp(min, max)
    }
}
