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
