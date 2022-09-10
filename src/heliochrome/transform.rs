use super::maths::mat3;

pub struct Transform {
    pub matrix: mat3,
    pub inverse: mat3,
}

impl Transform {
    pub fn new(matrix: mat3) -> Transform {
        Transform {
            matrix,
            inverse: matrix.inverse(),
        }
    }
}
