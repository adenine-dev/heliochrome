use super::maths::mat3;

#[derive(Debug, Clone)]
pub struct Transform {
    pub matrix: mat3,
    pub inverse: mat3,
    pub normal_matrix: mat3,
}

impl Transform {
    pub fn new(matrix: mat3) -> Transform {
        let inverse = matrix.inverse();
        Transform {
            matrix,
            inverse,
            normal_matrix: inverse.transposed(),
        }
    }
}
