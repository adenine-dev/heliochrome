use indicatif::ProgressBar;

use super::{
    hittables::{Hit, Hittable, AABB},
    maths::Ray,
};

const INVALID_IDX: usize = usize::MAX;

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
    ) -> Option<Hit> {
        if !self.bounds.hits(ray, t_min, t_max) {
            return None;
        }

        if self.idx != INVALID_IDX {
            return hittables[self.idx].hit(ray, t_min, t_max);
        }

        let mut hit = None;
        for child in self.children {
            let h = nodes[child].hit(nodes, hittables, ray, t_min, t_max);
            if h.is_some() {
                hit = h;
                t_max = hit.as_ref().unwrap().t;
            }
        }

        hit
    }
}

pub struct BVH<T: Hittable> {
    hittables: Vec<T>,
    nodes: Vec<BVHNode>,
}

impl<T: Hittable> Hittable for BVH<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.nodes
            .last()?
            .hit(&self.nodes, &self.hittables, ray, t_min, t_max)
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        Some(self.nodes.last()?.bounds)
    }
}

impl<T: Hittable> BVH<T> {
    pub fn new(hittables: Vec<T>) -> Self {
        let mut nodes = Vec::with_capacity(hittables.len() * 2 - 1);
        let mut available = vec![false; hittables.len() * 2 - 1];

        for i in 0..hittables.len() {
            nodes.push(BVHNode {
                bounds: hittables[i].make_bounding_box().unwrap(),
                idx: i,
                children: [INVALID_IDX; 2],
            });
            available[i] = true;
        }

        let pb = ProgressBar::new((hittables.len() * 2 - 1) as u64);

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
                            if s < best {
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

            pb.inc(1);
        }

        Self { hittables, nodes }
    }
}
