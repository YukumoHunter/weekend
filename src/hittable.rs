use super::{aabb::Aabb, material::Material, moving::Timespan, ray::Ray, texture::Uv};
use nalgebra::Vector3;

pub struct HitRecord<'a> {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub material: &'a dyn Material,
    pub uv: Uv,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, timespan: Timespan) -> Option<Aabb>;
}

#[derive(Default)]
pub struct HittableList {
    pub raw: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.raw.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest = t_max;

        for hittable in self.raw.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, closest) {
                closest = hit.t;
                record = Some(hit);
            }
        }

        record
    }

    fn bounding_box(&self, timespan: Timespan) -> Option<Aabb> {
        if self.raw.is_empty() {
            return None;
        }

        let mut output_box: Option<Aabb> = None;
        for hittable in &self.raw {
            if let Some(temp_box) = hittable.bounding_box(timespan) {
                if let Some(curr_box) = output_box {
                    output_box = Some(curr_box.surrounding_box(&temp_box))
                } else {
                    output_box = Some(temp_box);
                }
            } else {
                return None;
            }
        }
        output_box
    }
}
