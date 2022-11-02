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

use super::{triangle_bounding_box, Hit, Hittable, Intersect, Triangle, AABB};
use crate::{
    bvh::BVH,
    maths::{vec3, Ray},
};

#[derive(Clone)]
pub struct Mesh {
    tris: BVH<Triangle>,
    normals: Option<Vec<[vec3; 3]>>,
}

impl Mesh {
    pub fn new(positions: &[vec3], indices: &Vec<u32>, vnormals: &[vec3]) -> Self {
        let mut tris = vec![Triangle::default(); indices.len() / 3];
        let mut normals = if vnormals.is_empty() {
            None
        } else {
            Some(Vec::with_capacity(indices.len()))
        };
        for i in 0..indices.len() / 3 {
            tris[i] = Triangle::new([
                positions[indices[i * 3] as usize],
                positions[indices[i * 3 + 1] as usize],
                positions[indices[i * 3 + 2] as usize],
            ]);

            if let Some(normals) = &mut normals {
                normals.push([
                    vnormals[indices[i * 3] as usize],
                    vnormals[indices[i * 3 + 1] as usize],
                    vnormals[indices[i * 3 + 2] as usize],
                ]);
            }
        }
        Mesh {
            tris: BVH::new(tris),
            normals,
        }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if let Some((mut hit, idx)) = self.tris.hit_with_index(ray, t_min, t_max) {
            if let Some(normals) = &self.normals {
                let a = self.tris.hittables[idx].vertices[0];
                let b = self.tris.hittables[idx].vertices[1];
                let c = self.tris.hittables[idx].vertices[2];
                let p = hit.p;

                let v0 = b - a;
                let v1 = c - a;
                let v2 = p - a;
                let d00 = v0.dot(v0);
                let d01 = v0.dot(v1);
                let d11 = v1.dot(v1);
                let d20 = v2.dot(v0);
                let d21 = v2.dot(v1);
                let denom = d00 * d11 - d01 * d01;
                let v = (d11 * d20 - d01 * d21) / denom;
                let w = (d00 * d21 - d01 * d20) / denom;
                let u = 1.0 - v - w;

                hit.set_normal(
                    ray,
                    u * normals[idx][0] + v * normals[idx][1] + w * normals[idx][2],
                );
            }
            Some(hit)
        } else {
            None
        }
    }

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersect> {
        let (mut intersect, idx) = self.tris.intersect_with_index(ray, t_min, t_max)?;
        intersect.i = idx as u32;
        Some(intersect)
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
