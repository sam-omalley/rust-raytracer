use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;

use std::cmp::Ordering;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &Vec<Arc<dyn Hittable>>) -> Self {
        BvhNode::from_list(&objects, 0, objects.len())
    }

    fn from_list(src_objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut objects = src_objects.clone();
        let mut bbox = Aabb::empty();
        for obj_idx in start..end {
            bbox = Aabb::combine(&bbox, objects[obj_idx].bounding_box());
        }
        let axis = bbox.longest_axis();

        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            BvhNode::box_compare(a.clone(), b.clone(), axis)
        };

        let object_span = end - start;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects.as_mut_slice()[start..end].sort_by(comparator);

            let mid = start + object_span / 2;

            left = Arc::new(Self::from_list(&objects, start, mid));
            right = Arc::new(Self::from_list(&objects, mid, end));
        }

        BvhNode { left, right, bbox }
    }

    fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis_index: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        if a_axis_interval.min() <= b_axis_interval.min() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<(HitRecord, &Material)> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let left = self.left.hit(r, ray_t);
        let right = self.right.hit(
            r,
            Interval::new(
                ray_t.min(),
                if left.is_some() {
                    left.clone().unwrap().0.t
                } else {
                    ray_t.max()
                },
            ),
        );

        match (left, right) {
            (None, None) => None,
            (Some(lbox), None) => Some(lbox),
            (_, Some(rbox)) => Some(rbox)
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
