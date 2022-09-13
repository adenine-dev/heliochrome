use crate::heliochrome::maths::{vec3, Ray};

use super::{ray_triangle_intersection, Hit, Hittable};

pub struct Mesh {
    tris: Vec<[vec3; 3]>,
}

impl Mesh {
    pub fn new(positions: &Vec<vec3>, indices: &Vec<u32>) -> Self {
        let mut tris = vec![[vec3::default(); 3]; indices.len() / 3];
        for i in 0..indices.len() / 3 {
            tris[i] = [
                positions[indices[i * 3 + 0] as usize],
                positions[indices[i * 3 + 1] as usize],
                positions[indices[i * 3 + 2] as usize],
            ];
        }

        Mesh { tris }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<Hit> {
        let mut hit = None;
        self.tris.iter().for_each(|tri| {
            let h = ray_triangle_intersection(ray, tri, t_min, t_max);
            if h.is_some() {
                t_max = h.as_ref().unwrap().t;
                hit = h;
            }
        });

        hit
    }
}
