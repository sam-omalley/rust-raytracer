use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Bvh {
    bbox: Aabb,
    size: usize,
    contents: BvhContents,
}

#[derive(Debug)]
pub enum BvhContents {
    Node { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

impl Bvh {
    pub fn new(mut objs: Vec<Box<dyn Hittable>>) -> Self {
        // Note: though this BVH implementation is largely derived from Peter
        // Shirley's, it does *not* use the random axis selection and sort
        // routine, because it tends to fall into pathological cases.
        fn axis_range(objs: &[Box<dyn Hittable>], axis: usize) -> f32 {
            let range = objs.iter().fold(f32::MAX..f32::MIN, |range, o| {
                let bb = o.bounding_box();
                let min = bb.min[axis].min(bb.max[axis]);
                let max = bb.min[axis].max(bb.max[axis]);
                range.start.min(min)..range.end.max(max)
            });
            range.end - range.start
        }

        // Find the axis that has the greatest range for this set of objects.
        // TODO: Implement Axis enum
        let axis = {
            let mut ranges = [
                (0, axis_range(&objs, 0)),
                (1, axis_range(&objs, 1)),
                (2, axis_range(&objs, 2)),
            ];
            // Note reversed comparison function, to sort descending:
            ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            ranges[0].0
        };

        // Sort objects along it by centroid. (Actually, by cenroid*2. This is equivalent and cheaper.)
        objs.sort_unstable_by(|a, b| {
            let abb = a.bounding_box();
            let bbb = b.bounding_box();
            let av = abb.min[axis] + abb.max[axis];
            let bv = bbb.min[axis] + bbb.max[axis];
            av.partial_cmp(&bv).unwrap()
        });

        match objs.len() {
            0 => panic!("Can't create a BVH from zero objects."),
            1 => Bvh {
                bbox: objs[0].bounding_box(),
                size: 1,
                contents: BvhContents::Leaf(objs.pop().unwrap()),
            },
            _ => {
                // Divide space at the median point of the selected axis.
                let right = Box::new(Bvh::new(objs.drain(objs.len() / 2..).collect()));
                let left = Box::new(Bvh::new(objs));

                Bvh {
                    bbox: Aabb::combine(&left.bbox, &right.bbox),
                    size: left.size + right.size,
                    contents: BvhContents::Node { left, right },
                }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit<'a>(&'a self, r: &Ray, mut ray_t: Interval) -> Option<HitRecord<'a>> {
        if self.bbox.hit(r, ray_t) {
            match &self.contents {
                BvhContents::Node { left, right } => {
                    let hit_left = left.hit(r, ray_t);

                    // Don't bother searching past the left ht in the right space.
                    if let Some(rec) = &hit_left {
                        ray_t.max = rec.t;
                    }

                    let hit_right = right.hit(r, ray_t);

                    match (hit_left, hit_right) {
                        (h, None) | (None, h) => h,
                        (Some(hl), Some(hr)) => {
                            if hl.t < hr.t {
                                Some(hl)
                            } else {
                                Some(hr)
                            }
                        }
                    }
                }
                BvhContents::Leaf(obj) => obj.hit(r, ray_t),
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
