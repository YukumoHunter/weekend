use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
    moving::{Timespan, Trajectory},
    ray::Ray,
    texture::Uv,
};
use nalgebra::Vector3;
use std::sync::Arc;

pub struct Sphere<M: Material> {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Arc<M>,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3<f32>, radius: f32, material: Arc<M>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn uv_at(point: &Vector3<f32>) -> Uv {
        let theta = -point.y.acos();
        let phi = -point.z.atan2(point.x) + std::f32::consts::PI;

        let u = phi / (2. * std::f32::consts::PI);
        let v = theta / std::f32::consts::PI;

        Uv::new(u, v)
    }
}

impl<M: Material> Hittable for Sphere<M> {
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
        let outward_normal = (intersection_point - self.center) / self.radius;

        Some(HitRecord {
            t: root,
            p: intersection_point,
            normal: outward_normal,
            material: self.material.as_ref(),
            uv: Self::uv_at(&outward_normal),
        })
    }

    fn bounding_box(&self, _timespan: Timespan) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vector3::from_element(self.radius),
            self.center + Vector3::from_element(self.radius),
        ))
    }
}

pub struct MovingSphere<M: Material> {
    pub movement: Trajectory<Vector3<f32>>,
    pub radius: f32,
    pub material: Arc<M>,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(movement: Trajectory<Vector3<f32>>, radius: f32, material: Arc<M>) -> Self {
        Self {
            movement,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vector3<f32> {
        self.movement.from
            + ((time - self.movement.timespan.start) / (self.movement.timespan.difference()))
                * (self.movement.to - self.movement.from)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
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
        let outward_normal = (intersection_point - center) / self.radius;
        let uv = Sphere::<M>::uv_at(&outward_normal);

        Some(HitRecord {
            t: root,
            p: intersection_point,
            normal: outward_normal,
            material: self.material.as_ref(),
            uv,
        })
    }

    fn bounding_box(&self, timespan: Timespan) -> Option<Aabb> {
        let box0 = Aabb::new(
            Self::center(self, timespan.start) - Vector3::from_element(self.radius),
            Self::center(self, timespan.start) + Vector3::from_element(self.radius),
        );

        let box1 = Aabb::new(
            Self::center(self, timespan.end) - Vector3::from_element(self.radius),
            Self::center(self, timespan.end) + Vector3::from_element(self.radius),
        );

        Some(box0.surrounding_box(&box1))
    }
}
