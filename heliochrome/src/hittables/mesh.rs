// use super::{triangle_bounding_box, Hit, Hittable, Triangle, AABB};
// use crate::{
//     bvh::BVH,
//     maths::{vec3, Ray},
// };

// #[derive(Clone)]
// pub struct Mesh {
//     bvh: BVH<Triangle>,
// }

// impl Mesh {
//     pub fn new(positions: &[vec3], indices: &Vec<u32>) -> Self {
//         let mut tris = vec![Triangle::default(); indices.len() / 3];
//         for i in 0..indices.len() / 3 {
//             tris[i] = Triangle::new([
//                 positions[indices[i * 3] as usize],
//                 positions[indices[i * 3 + 1] as usize],
//                 positions[indices[i * 3 + 2] as usize],
//             ]);
//         }
//         let (bvh, _) = BVH::new(tris);
//         Mesh { bvh }
//     }
// }

// impl Hittable for Mesh {
//     fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
//         self.bvh.hit(ray, t_min, t_max)
//     }

//     fn make_bounding_box(&self) -> Option<AABB> {
//         let mut min = vec3::splat(f32::INFINITY);
//         let mut max = vec3::splat(-f32::INFINITY);

//         for t in self.bvh.hittables.iter() {
//             let aabb = t.make_bounding_box().unwrap();
//             min = min.min(&aabb.min);
//             max = max.max(&aabb.max);
//         }

//         Some(AABB::new(
//             min - vec3::splat(0.001),
//             max + vec3::splat(0.001),
//         ))
//     }
// }

use super::{triangle_bounding_box, Hit, Hittable, Triangle, AABB};
use crate::{
    bvh::BVH,
    maths::{vec3, Ray},
};

#[derive(Clone)]
pub struct Mesh {
    tris: BVH<Triangle>,
}

impl Mesh {
    pub fn new(positions: &[vec3], indices: &Vec<u32>) -> Self {
        let mut tris = vec![Triangle::default(); indices.len() / 3];
        for i in 0..indices.len() / 3 {
            tris[i] = Triangle::new([
                positions[indices[i * 3] as usize],
                positions[indices[i * 3 + 1] as usize],
                positions[indices[i * 3 + 2] as usize],
            ]);
        }
        Mesh {
            tris: BVH::new(tris),
        }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.tris.hit(ray, t_min, t_max)
    }

    fn make_bounding_box(&self) -> AABB {
        let mut min = vec3::splat(f32::INFINITY);
        let mut max = vec3::splat(-f32::INFINITY);

        for t in self.tris.hittables.iter() {
            let aabb = triangle_bounding_box(&t.vertices);
            min = min.min(&aabb.min);
            max = max.max(&aabb.max);
        }

        AABB::new(min - vec3::splat(0.001), max + vec3::splat(0.001))
    }
}
