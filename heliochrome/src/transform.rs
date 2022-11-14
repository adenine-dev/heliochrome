use std::error::Error;

use crate::{
    hittables::AABB,
    loader::{parse_into, FromHCY},
    maths::{mat4, vec3, vec4, Ray},
};

#[derive(Debug, Clone)]
pub struct Transform {
    pub matrix: mat4,
    pub inverse: mat4,
    // pub normal_matrix: mat4,
}

impl Transform {
    pub fn new(matrix: mat4) -> Transform {
        Transform {
            matrix,
            inverse: matrix.inverse(),
        }
    }

    pub fn trans_ray(&self, r: &Ray) -> Ray {
        Ray::new(
            (self.inverse * vec4::from_vec3(r.origin, 1.0)).to_vec3(),
            (self.inverse * vec4::from_vec3(r.direction, 0.0)).to_vec3(),
        )
    }

    pub fn trans_pos(&self, v: &vec3) -> vec3 {
        // NOTE: no projection is done so we don't need to divide by w
        (self.inverse * vec4::from_vec3(*v, 1.0)).to_vec3()
    }

    pub fn trans_point(&self, v: &vec3) -> vec3 {
        // NOTE: no projection is done so we don't need to divide by w
        (self.matrix * vec4::from_vec3(*v, 1.0)).to_vec3()
    }

    pub fn trans_dir(&self, v: &vec3) -> vec3 {
        (self.inverse * vec4::from_vec3(*v, 0.0)).to_vec3()
    }

    pub fn trans_normal(&self, v: &vec3) -> vec3 {
        (self.inverse.trans_mul(vec4::from_vec3(*v, 0.0)))
            .to_vec3()
            .normalize()
    }

    pub fn trans_aabb(&self, aabb: &AABB) -> AABB {
        let mut min = self.matrix[3].to_vec3();
        let mut max = self.matrix[3].to_vec3();

        for i in 0..3 {
            for j in 0..3 {
                let a = self.matrix[i][j] * aabb.min[i];
                let b = self.matrix[i][j] * aabb.max[i];
                if a < b {
                    min[j] += a;
                    max[j] += b;
                } else {
                    min[j] += b;
                    max[j] += a;
                }
            }
        }

        AABB::new(min, max)
    }
}

impl FromHCY for Transform {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut translate = mat4::identity();
        let mut rotate = mat4::identity();
        let mut scale = mat4::identity();

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "translate" => translate = mat4::translate(parse_into(value)?),
                "rotate" => rotate = mat4::rotate_deg(parse_into(value)?),
                "scale" => scale = mat4::scale(parse_into(value)?),
                _ => {}
            }
        }

        Ok(Transform::new(translate * rotate * scale))
    }
}
