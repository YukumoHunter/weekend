use rand::Rng;

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    moving::Timespan,
    ray::Ray,
};
use std::cmp::Ordering;

pub enum BvhNode {
    Branch {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
    },
    Leaf(Box<dyn Hittable>),
}

pub struct BvhTree {
    node: BvhNode,
    bounding_box: Aabb,
}

impl BvhTree {
    pub fn new(mut hittables: Vec<Box<dyn Hittable>>, timespan: Timespan) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..=2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => unreachable!(),
        };

        hittables.sort_unstable_by(|a, b| comparator(a.as_ref(), b.as_ref()));

        let span = hittables.len();
        match span {
            1 => {
                let first = hittables.remove(0);
                let bounding_box = first
                    .bounding_box(timespan)
                    .expect("No bounding box for BvhNode");

                BvhTree {
                    node: BvhNode::Leaf(first),
                    bounding_box,
                }
            }
            _ => {
                let mid = span / 2;
                let mut left_hittables = hittables;
                let right_hittables = left_hittables.drain(..mid).collect::<Vec<_>>();

                let new_left = Self::new(left_hittables, timespan);
                let new_right = Self::new(right_hittables, timespan);

                let bounding_box = new_left
                    .bounding_box(timespan)
                    .expect("No bounding box for BvhNode")
                    .surrounding_box(
                        &new_right
                            .bounding_box(timespan)
                            .expect("No bounding box for BvhNode"),
                    );

                BvhTree {
                    node: BvhNode::Branch {
                        left: Box::new(new_left),
                        right: Box::new(new_right),
                    },
                    bounding_box,
                }
            }
        }
    }

    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> Ordering {
        let a_box = a
            .bounding_box(Timespan::new(0., 0.))
            .expect("No bounding box in BvhNode constructor");
        let b_box = b
            .bounding_box(Timespan::new(0., 0.))
            .expect("No bounding box in BvhNode constructor");

        a_box.min[axis].total_cmp(&b_box.min[axis])
    }

    fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhTree {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let mut hit = None;

        match &self.node {
            BvhNode::Leaf(leaf) => leaf.hit(ray, t_min, t_max),
            BvhNode::Branch { left, right } => {
                if let Some(hit_left) = left.hit(ray, t_min, t_max) {
                    t_max = hit_left.t;
                    hit = Some(hit_left);
                }

                if let Some(hit_right) = right.hit(ray, t_min, t_max) {
                    Some(hit_right)
                } else {
                    hit
                }
            }
        }
    }

    fn bounding_box(&self, _timespan: Timespan) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
