use std::cmp::Ordering;
use std::rc::Rc;

use crate::geometry::{Axis, HitRecord, Hittable, HittableList, AABB};
use crate::optics::Ray;
use crate::utils::Interval;

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(src_objects: &[Rc<dyn Hittable>], start: usize, end: usize) -> Self {
        let mut objects = src_objects.to_vec();
        let axis = Axis::from_idx(rand::random::<usize>() % 3);

        let comparator = Self::comparator(axis);

        let object_span = end - start;

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

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
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::new(&objects, start, mid));
            right = Rc::new(BvhNode::new(&objects, mid, end));
        }

        let left_box = left.bounding_box();
        let right_box = right.bounding_box();

        Self {
            bbox: AABB::wrap_boxes(left_box, right_box),
            left,
            right,
        }
    }

    pub fn from_hittable_list(list: &HittableList) -> Self {
        Self::new(&list.objects, 0, list.objects.len())
    }
}

impl BvhNode {
    fn comparator(axis: Axis) -> impl Fn(&Rc<dyn Hittable>, &Rc<dyn Hittable>) -> Ordering {
        move |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
            let box_a = a.bounding_box();
            let box_b = b.bounding_box();

            box_a
                .axis(&axis)
                .min
                .partial_cmp(&box_b.axis(&axis).min)
                .unwrap()
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t) {
            return false;
        }

        let hit_left = self.left.as_ref().hit(r, t, rec);
        let hit_right = self.right.as_ref().hit(
            r,
            Interval::new(t.min, if hit_left { rec.t } else { t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn depth(&self) -> usize {
        1 + std::cmp::max(self.left.as_ref().depth(), self.right.as_ref().depth())
    }
}
