use super::maths::mat3;

#[derive(Debug)]
pub struct Transform {
    pub inverse: mat3,
    pub normal_matrix: mat3,
}

impl Transform {
    pub fn new(matrix: mat3) -> Transform {
        let inverse = matrix.inverse();
        Transform {
            inverse,
            normal_matrix: inverse.transposed(),
        }
    }
}
