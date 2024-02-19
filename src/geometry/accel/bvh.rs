use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::geometry::{Axis, HitRecord, Hittable, HittableList, AABB};
use crate::optics::Ray;
use crate::utils::Interval;

pub struct BvhNode {
    pub left: Option<Rc<BvhNode>>,
    pub right: Option<Rc<BvhNode>>,
    pub leaf: Option<Rc<dyn Hittable>>,
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
    pub fn new(src_objects: &[Rc<dyn Hittable>], start: usize, end: usize) -> Self {
        let mut objects = src_objects.to_vec();
        let axis = Axis::from_idx(rand::random::<usize>() % 3);

        let comparator = Self::comparator(axis);

        let object_span = end - start;

        let mut left: Option<Rc<BvhNode>> = None;
        let mut right: Option<Rc<BvhNode>> = None;
        let mut leaf: Option<Rc<dyn Hittable>> = None;

        if object_span == 1 {
            leaf = Some(objects[start].clone());
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            left = Some(Rc::new(BvhNode::new(&objects, start, mid)));
            right = Some(Rc::new(BvhNode::new(&objects, mid, end)));
        }

        let bbox = if let Some(_leaf) = leaf.clone() {
            *_leaf.as_ref().bounding_box()
        } else {
            AABB::wrap_boxes(&left.as_ref().unwrap().bbox, &right.as_ref().unwrap().bbox)
        };

        Self {
            left,
            right,
            leaf,
            bbox,
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

        if self.leaf.is_some() {
            return self.leaf.as_ref().unwrap().hit(r, t, rec);
        }

        let hit_left = self.left.as_ref().unwrap().hit(r, t, rec);
        let hit_right = self.right.as_ref().unwrap().hit(
            r,
            Interval::new(t.min, if hit_left { rec.t } else { t.max }),
            rec,
        );

        hit_left || hit_right
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
