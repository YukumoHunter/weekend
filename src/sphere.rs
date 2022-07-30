use super::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};
use nalgebra::Vector3;
use std::sync::Arc;

pub struct Sphere<'a> {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Arc<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vector3<f32>, radius: f32, material: Arc<dyn Material + 'a>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the neareast root in an acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let intersection_point = ray.at(root);

        Some(HitRecord {
            t: root,
            p: intersection_point,
            normal: (intersection_point - self.center) / self.radius,
            material: self.material.clone(),
        })
    }
}
