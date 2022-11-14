use std::error::Error;

use super::{BounceInfo, Hittable, Intersection, Triangle, AABB};
use crate::{
    accel::{Accel, Accelerator},
    load_obj,
    loader::{parse_into, FromHCY},
    maths::{vec3, Ray},
};

#[derive(Clone, Debug)]
pub struct Mesh {
    tris: Accel<Triangle>,
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
            tris: Accel::new(tris),
            normals,
        }
    }
}

impl Hittable for Mesh {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let (mut intersect, idx) = self.tris.intersect_with_index(ray, t_min, t_max)?;
        intersect.i = idx as u32;
        Some(intersect)
    }

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo {
        let idx = intersection.i as usize;
        let mut bounce_info = BounceInfo::new(ray, intersection.t, vec3::default());
        bounce_info.set_normal(
            ray,
            if let Some(normals) = &self.normals {
                let a = self.tris.hittables[idx].vertices[0];
                let b = self.tris.hittables[idx].vertices[1];
                let c = self.tris.hittables[idx].vertices[2];

                let v0 = b - a;
                let v1 = c - a;
                let v2 = bounce_info.p - a;
                let d00 = v0.dot(v0);
                let d01 = v0.dot(v1);
                let d11 = v1.dot(v1);
                let d20 = v2.dot(v0);
                let d21 = v2.dot(v1);
                let denom = d00 * d11 - d01 * d01;
                let v = (d11 * d20 - d01 * d21) / denom;
                let w = (d00 * d21 - d01 * d20) / denom;
                let u = 1.0 - v - w;

                u * normals[idx][0] + v * normals[idx][1] + w * normals[idx][2]
            } else {
                self.tris.hittables[idx].normal()
            },
        );

        bounce_info
    }

    fn make_bounding_box(&self) -> AABB {
        let mut min = vec3::splat(f32::INFINITY);
        let mut max = vec3::splat(-f32::INFINITY);

        for t in self.tris.hittables.iter() {
            let aabb = t.make_bounding_box();
            min = min.min(&aabb.min);
            max = max.max(&aabb.max);
        }

        AABB::new(min - vec3::splat(0.001), max + vec3::splat(0.001))
    }
}

impl FromHCY for Mesh {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut path = None;
        let mut idx = 0;

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "path" => path = Some(value.trim().to_owned()),
                "idx" => idx = parse_into(value)?,
                _ => {}
            }
        }

        let path = path.ok_or("missing required key `path`")?;
        let meshes = load_obj::load_obj(path)?;
        let mesh = meshes.get(idx).ok_or(format!(
            "invalid mesh index {idx}, obj file has {} meshes",
            meshes.len(),
        ))?;

        Ok(Self::new(&mesh.vertices, &mesh.indices, &mesh.normals))
    }
}
