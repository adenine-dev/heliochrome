use super::{
    hittables::{Hit, Hittable, AABB},
    maths::Ray,
};
use crate::maths::vec3;

const INVALID_IDX: usize = usize::MAX;

#[derive(Clone)]
struct BVHNode {
    bounds: AABB,
    idx: usize,
    children: [usize; 2],
}

impl BVHNode {
    pub fn hit<T: Hittable>(
        &self,
        nodes: &Vec<BVHNode>,
        hittables: &Vec<T>,
        ray: &Ray,
        t_min: f32,
        mut t_max: f32,
    ) -> Option<(Hit, usize)> {
        if !self.bounds.hits(ray, t_min, t_max) {
            return None;
        }

        if self.idx != INVALID_IDX {
            if let Some(hit) = hittables[self.idx].hit(ray, t_min, t_max) {
                return Some((hit, self.idx));
            }
            return None;
        }

        let mut hit = None;
        for child in self.children {
            let h = nodes[child].hit(nodes, hittables, ray, t_min, t_max);
            if h.is_some() {
                hit = h;
                t_max = hit.as_ref().unwrap().0.t;
            }
        }

        hit
    }
}

#[derive(Clone)]
pub struct BVH<T: Hittable> {
    pub hittables: Vec<T>,
    nodes: Vec<BVHNode>,
}

impl<T: Hittable> Hittable for BVH<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        // return self.hittables[0].hit(ray, t_min, t_max);
        if let Some((hit, _)) =
            self.nodes
                .last()?
                .hit(&self.nodes, &self.hittables, ray, t_min, t_max)
        {
            return Some(hit);
        }

        None
    }

    fn make_bounding_box(&self) -> AABB {
        if let Some(last) = self.nodes.last() {
            last.bounds
        } else {
            AABB::new(vec3::splat(-0.0001), vec3::splat(0.0001))
        }
    }
}

impl<T: Hittable> BVH<T> {
    pub fn new(hittables: Vec<T>) -> Self {
        if hittables.is_empty() {
            return BVH {
                hittables,
                nodes: vec![],
            };
        }

        let mut nodes = Vec::with_capacity(hittables.len() * 2 - 1);
        let mut available = vec![false; hittables.len() * 2 - 1];
        for i in 0..hittables.len() {
            nodes.push(BVHNode {
                bounds: hittables[i].make_bounding_box(),
                idx: i,
                children: [INVALID_IDX; 2],
            });
            available[i] = true;
        }

        while nodes.len() < hittables.len() * 2 - 1 {
            let mut best = f32::INFINITY;
            let mut bounds = AABB::default();
            let mut l = INVALID_IDX;
            let mut r = INVALID_IDX;
            for i in 0..nodes.len() {
                if available[i] {
                    for j in 0..nodes.len() {
                        if available[j] && i != j {
                            let b = AABB::surrounding(&nodes[i].bounds, &nodes[j].bounds);
                            let s = b.surface_area();
                            if s <= best {
                                best = s;
                                bounds = b;
                                l = i;
                                r = j
                            }
                        }
                    }
                }
            }

            available[l] = false;
            available[r] = false;

            nodes.push(BVHNode {
                bounds,
                idx: INVALID_IDX,
                children: [l, r],
            });
            available[nodes.len() - 1] = true;
        }

        Self { hittables, nodes }
    }

    pub fn hit_with_index(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, usize)> {
        self.nodes
            .last()?
            .hit(&self.nodes, &self.hittables, ray, t_min, t_max)
    }

    pub fn hit_obj(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Hit, &T)> {
        if let Some((hit, idx)) =
            self.nodes
                .last()?
                .hit(&self.nodes, &self.hittables, ray, t_min, t_max)
        {
            return Some((hit, &self.hittables[idx]));
        }

        None
    }
}
