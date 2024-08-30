use crate::geometry::{accel::AABB, Axis, HitRecord, Hittable, HittableList};
use crate::optics::Ray;
use crate::utils::Interval;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;

const CUTOFF: usize = 4;
const MAX_DEPTH: u32 = 16;

#[derive(Debug, Clone)]
pub struct Bvh {
    tree: Vec<BvhNode>,
}
#[derive(Debug, Clone)]
enum BvhNode {
    Leaf(Box<HittableList>),
    Inner(AABB, usize, usize),
}

impl BvhNode {
    fn bounding_box(&self) -> &AABB {
        match self {
            Self::Leaf(leaf) => leaf.bounding_box(),
            Self::Inner(bbox, _, _) => bbox,
        }
    }

    fn update_child_idx(&mut self, idx: usize, child_type: ChildType) {
        match self {
            Self::Leaf(_) => panic!("Leaf nodes have no children"),
            Self::Inner(_, left_idx, right_idx) => match child_type {
                ChildType::Root => return,
                ChildType::Left => *left_idx = idx,
                ChildType::Right => *right_idx = idx,
            },
        }
    }
}

enum ChildType {
    Root,
    Left,
    Right,
}

impl Bvh {
    pub fn new(objects: &mut [Box<dyn Hittable>]) -> Self {
        let mut tree = Vec::new();
        let mut covered = vec![false; objects.len()];

        let mut queue = VecDeque::new();
        queue.push_back((0, ChildType::Root, 0, 0, objects.len()));

        let mut max_depth: u32 = 0;
        let mut num_leafs: u32 = 0;
        let mut leaf_sum: u32 = 0;
        let mut depth_sum: u32 = 0;

        while let Some((parent_idx, child_type, depth, start, end)) = queue.pop_front() {
            let curr_idx = tree.len();

            let object_span = end - start;
            if object_span <= CUTOFF || depth >= MAX_DEPTH {
                let leaf = HittableList::new(objects[start..end].to_vec());
                tree.push(BvhNode::Leaf(Box::new(leaf)));

                // Stats
                covered[start..end].iter_mut().for_each(|c| *c = true);
                num_leafs += 1;
                depth_sum += depth;
                leaf_sum += object_span as u32;
                max_depth = max_depth.max(depth);
            } else {
                let bbox = AABB::wrap_objects(&objects[start..end]);
                tree.push(BvhNode::Inner(bbox.clone(), usize::MAX, usize::MAX));

                let axis = bbox.longest_axis();
                objects[start..end].sort_by(Self::object_comparator(axis));

                let mut min_split = 0;
                let mut min_cost = f64::INFINITY;

                // Determine Best Split
                for split in (start + 1..end).step_by(1.max(object_span / 16)) {
                    let left = AABB::wrap_objects(&objects[start..split]);
                    let right = AABB::wrap_objects(&objects[split..end]);
                    let cost = left.surface_area() * (split - start) as f64
                        + right.surface_area() * (end - split) as f64;
                    if cost < min_cost {
                        min_split = split;
                        min_cost = cost;
                    }
                }

                // Recursion
                queue.push_back((curr_idx, ChildType::Left, depth + 1, start, min_split));
                queue.push_back((curr_idx, ChildType::Right, depth + 1, min_split, end));
            }

            // Update Parent
            tree[parent_idx].update_child_idx(curr_idx, child_type);
        }

        let mut covered_count = 0;
        for c in covered.iter() {
            if *c {
                covered_count += 1;
            }
        }

        println!("Covered {}/{} objects", covered_count, objects.len());

        Self::check_tree(&tree);

        println!(
            "BVH Construction: {} leafs, {:.2} avg objects, {} max depth, {:.2} avg depth",
            num_leafs,
            leaf_sum as f64 / num_leafs as f64,
            max_depth,
            depth_sum as f64 / num_leafs as f64
        );

        Self { tree }
    }

    fn check_tree(tree: &Vec<BvhNode>) {
        let mut stack = VecDeque::new();
        stack.push_back(0);

        let mut reached = vec![false; tree.len()];

        while let Some(i) = stack.pop_back() {
            reached[i] = true;
            match &tree[i] {
                BvhNode::Leaf(_) => continue,
                BvhNode::Inner(_, left_idx, right_idx) => {
                    stack.push_back(*left_idx);
                    stack.push_back(*right_idx);
                }
            }
        }

        let mut reached_count = 0;
        for r in reached.iter() {
            if *r {
                reached_count += 1;
            }
        }
        println!("Reached {}/{} nodes", reached_count, tree.len());
    }

    fn object_comparator(
        axis: Axis,
    ) -> impl Fn(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
        move |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| {
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

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        if !self.tree[0].bounding_box().hit(r, t, rec) {
            return false;
        }

        // rec.mat = Rc::new(Lambertian::from_albedo(Color::new(1.0, 0.0, 0.0)));
        // return true;

        let mut stack = VecDeque::new();
        stack.push_back(0);

        let mut hit_anything = false;
        let mut closest_so_far = t.max;

        while let Some(i) = stack.pop_back() {
            let t_closest = Interval::new(t.min, closest_so_far);

            match &self.tree[i] {
                BvhNode::Leaf(leaf) => {
                    if leaf.hit(r, t_closest, rec) {
                        hit_anything = true;
                        closest_so_far = rec.t;
                    }
                }
                BvhNode::Inner(_, left_idx, right_idx) => {
                    let mut rec_left = rec.clone();
                    let mut rec_right = rec.clone();

                    let left_hit =
                        self.tree[*left_idx]
                            .bounding_box()
                            .hit(r, t_closest, &mut rec_left);

                    let right_hit =
                        self.tree[*right_idx]
                            .bounding_box()
                            .hit(r, t_closest, &mut rec_right);

                    if left_hit && right_hit {
                        let (first_idx, second_idx) = if rec_left.t < rec_right.t {
                            (right_idx, left_idx)
                        } else {
                            (left_idx, right_idx)
                        };

                        stack.push_back(*first_idx);
                        stack.push_back(*second_idx);
                    } else if left_hit {
                        stack.push_back(*left_idx);
                    } else if right_hit {
                        stack.push_back(*right_idx);
                    }
                }
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &AABB {
        self.tree[0].bounding_box()
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
