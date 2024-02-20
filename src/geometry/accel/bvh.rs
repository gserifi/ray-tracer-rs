use rand::Rng;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::geometry::{Axis, HitRecord, Hittable, HittableList, AABB};
use crate::optics::Ray;
use crate::utils::Interval;

const CUTOFF: usize = 1024;
const PADDING: usize = 64;

pub struct BvhNode {
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>,
    pub leaf: Option<Box<HittableList>>,
    bbox: AABB,
}

impl Debug for BvhNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.leaf.is_some() {
            f.debug_struct(format!("BvhNode<{}>", self.height()).as_str())
                .field("leaf", &self.leaf)
                .finish()
        } else {
            f.debug_struct(format!("BvhNode<{}>", self.height()).as_str())
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }
}

impl BvhNode {
    pub fn new(src_objects: &mut [Rc<dyn Hittable>]) -> Self {
        let bbox = AABB::wrap_objects(src_objects);
        let axis = bbox.longest_axis();

        let comparator = Self::comparator(axis);

        let object_span = src_objects.len();

        let mut left: Option<Box<BvhNode>> = None;
        let mut right: Option<Box<BvhNode>> = None;
        let mut leaf: Option<Box<HittableList>> = None;

        if object_span < CUTOFF {
            leaf = Some(Box::new(HittableList::new(src_objects.to_vec())));
        } else {
            src_objects.sort_by(comparator);

            let mid = rand::thread_rng().gen_range(PADDING..(object_span - 1 - PADDING));
            let (left_slice, right_slice) = src_objects.split_at_mut(mid);
            left = Some(Box::new(BvhNode::new(left_slice)));
            right = Some(Box::new(BvhNode::new(right_slice)));
        }

        Self {
            left,
            right,
            leaf,
            bbox,
        }
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

    pub fn leaf_count(&self) -> usize {
        if let Some(leaf) = &self.leaf {
            leaf.objects.len()
        } else {
            self.left.as_ref().unwrap().leaf_count() + self.right.as_ref().unwrap().leaf_count()
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t) {
            return false;
        }

        if let Some(leaf) = self.leaf.as_ref() {
            leaf.hit(r, t, rec)
        } else {
            let hit_left = self.left.as_ref().unwrap().hit(r, t, rec);
            let hit_right = self.right.as_ref().unwrap().hit(
                r,
                Interval::new(t.min, if hit_left { rec.t } else { t.max }),
                rec,
            );

            hit_left || hit_right
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn height(&self) -> usize {
        if self.leaf.is_some() {
            1
        } else {
            1 + self
                .left
                .as_ref()
                .unwrap()
                .height()
                .max(self.right.as_ref().unwrap().height())
        }
    }
}
