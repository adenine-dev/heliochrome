use crate::{
    hittables::AABB,
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

        // let mut min = vec3::splat(f32::INFINITY);
        // let mut max = vec3::splat(-f32::INFINITY);

        // for i in 0..=1 {
        //     for j in 0..=1 {
        //         for k in 0..=1 {
        //             let p = (self.matrix
        //                 * vec4::from_vec3(
        //                     aabb.min * vec3::new((1 - i) as f32, (1 - j) as f32, (1 - k) as f32)
        //                         + aabb.max * vec3::new(i as f32, j as f32, k as f32),
        //                     1.0,
        //                 ))
        //             .to_vec3();
        //             min = min.min(&p);
        //             max = max.max(&p);
        //         }
        //     }
        // }
        AABB::new(min, max)
    }
}
