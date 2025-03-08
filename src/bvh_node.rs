use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

use std::cmp::Ordering;

enum BvhNode {
    Branch { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

pub struct Bvh {
    tree: BvhNode,
    bbox: Aabb,
}

impl Bvh {
    pub fn new(mut hittable: Vec<Box<dyn Hittable>>) -> Self {
        fn box_compare(
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box();
                let b_bbox = b.bounding_box();
                let ac = a_bbox.min()[axis] + a_bbox.max()[axis];
                let bc = b_bbox.min()[axis] + b_bbox.max()[axis];
                ac.partial_cmp(&bc).unwrap()
            }
        }

        fn axis_range(hittable: &Vec<Box<dyn Hittable>>, axis: usize) -> f64 {
            let (min, max) = hittable
                .iter()
                .fold((f64::MAX, f64::MIN), |(bmin, bmax), hit| {
                    let bbox = hit.bounding_box();
                    (bmin.min(bbox.min()[axis]), bmax.max(bbox.max()[axis]))
                });
            max - min
        }

        let axis_ranges: Vec<(usize, f64)> =
            (0..3).map(|a| (a, axis_range(&hittable, a))).collect();

        let axis = axis_ranges[0].0;

        hittable.sort_unstable_by(box_compare(axis));
        let len = hittable.len();
        match len {
            0 => panic!("No elements in scene"),
            1 => {
                let leaf = hittable.pop().unwrap();
                let bbox = leaf.bounding_box();
                Bvh {
                    tree: BvhNode::Leaf(leaf),
                    bbox,
                }
            }
            _ => {
                let right = Bvh::new(hittable.drain(len / 2..).collect());
                let left = Bvh::new(hittable);
                let bbox = Aabb::combine(&left.bbox, &right.bbox);
                Bvh {
                    tree: BvhNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, mut ray_t: Interval) -> Option<(HitRecord, &Material)> {
        if self.bbox.hit(r, ray_t) {
            match &self.tree {
                BvhNode::Leaf(leaf) => leaf.hit(r, ray_t),
                BvhNode::Branch { left, right } => {
                    let left = left.hit(r, ray_t);
                    if let Some((l_rec, _)) = &left {
                        ray_t.max = l_rec.t
                    };
                    let right = right.hit(r, ray_t);
                    if right.is_some() { right } else { left }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
