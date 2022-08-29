#[derive(Debug, Clone, Copy)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn new(width: impl Into<T>, height: impl Into<T>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}
