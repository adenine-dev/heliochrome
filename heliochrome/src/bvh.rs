use indicatif::{ProgressBar, ProgressStyle};

use super::{
    hittables::{BounceInfo, Hittable, AABB},
    maths::Ray,
};
use crate::{hittables::Intersection, maths::vec3};

const INVALID_IDX: usize = usize::MAX;

#[derive(Clone)]
struct BVHNode {
    bounds: AABB,
    idx: usize,
    children: [usize; 2],
}

impl BVHNode {
    pub fn intersect<T: Hittable>(
        &self,
        nodes: &Vec<BVHNode>,
        hittables: &Vec<T>,
        ray: &Ray,
        t_min: f32,
        mut t_max: f32,
    ) -> Option<(Intersection, usize)> {
        if !self.bounds.hits(ray, t_min, t_max) {
            return None;
        }

        if self.idx != INVALID_IDX {
            if let Some(intersection) = hittables[self.idx].intersect(ray, t_min, t_max) {
                return Some((intersection, self.idx));
            }
            return None;
        }

        let mut intersection = None;
        for child in self.children {
            let i = nodes[child].intersect(nodes, hittables, ray, t_min, t_max);
            if i.is_some() {
                intersection = i;
                t_max = intersection.as_ref().unwrap().0.t;
            }
        }

        intersection
    }
}

#[derive(Clone)]
pub struct BVH<T: Hittable> {
    pub hittables: Vec<T>,
    nodes: Vec<BVHNode>,
}

impl<T: Hittable> Hittable for BVH<T> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        if let Some((intersection, _)) =
            self.nodes
                .last()?
                .intersect(&self.nodes, &self.hittables, ray, t_min, t_max)
        {
            return Some(intersection);
        }

        None
    }

    fn get_bounce_info(&self, _ray: &Ray, _intersection: Intersection) -> BounceInfo {
        panic!("don't call get bounce info on a bvh");
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
        hittables.iter().enumerate().for_each(|(idx, hittable)| {
            nodes.push(BVHNode {
                bounds: hittable.make_bounding_box(),
                idx,
                children: [INVALID_IDX; 2],
            });
        });
        let pb = ProgressBar::new((hittables.len() * 2 - 1) as u64).with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7}")
                .unwrap(),
        );

        let mut active = (0..hittables.len()).collect::<Vec<_>>();

        while nodes.len() < hittables.len() * 2 - 1 {
            let mut best = f32::INFINITY;
            let mut bounds = AABB::default();
            let mut l = INVALID_IDX;
            let mut r = INVALID_IDX;

            let mut x = INVALID_IDX;
            let mut y = INVALID_IDX;
            for (x1, i) in active.iter().enumerate() {
                for (y1, j) in active[(x1 + 1)..].iter().enumerate() {
                    let b = AABB::surrounding(&nodes[*i].bounds, &nodes[*j].bounds);
                    let s = b.surface_area();
                    if s <= best {
                        best = s;
                        bounds = b;
                        l = *i;
                        r = *j;
                        x = x1;
                        y = y1 + x1 + 1;
                    }
                }
            }
            active[x] = nodes.len();
            active.swap_remove(y);
            nodes.push(BVHNode {
                bounds,
                idx: INVALID_IDX,
                children: [l, r],
            });

            pb.inc(1);
        }
        pb.finish();
        Self { hittables, nodes }
    }

    pub fn intersect_with_index(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(Intersection, usize)> {
        self.nodes
            .last()?
            .intersect(&self.nodes, &self.hittables, ray, t_min, t_max)
    }

    pub fn intersect_obj(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Intersection, &T)> {
        let (intersection, idx) =
            self.nodes
                .last()?
                .intersect(&self.nodes, &self.hittables, ray, t_min, t_max)?;
        Some((intersection, &self.hittables[idx]))
    }
}
