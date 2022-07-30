use super::{
    material::{random_color, Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    sphere::Sphere,
};
use nalgebra::Vector3;
use rand::Rng;
use std::sync::Arc;

pub struct HitRecord<'a> {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub material: Arc<dyn Material + 'a>,
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.hittables.push(Box::new(hittable));
    }

    pub fn example_few() -> Self {
        let mut world = Self::default();

        let mat_ground = Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.)));
        let mat_center = Arc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.3)));
        let mat_left = Arc::new(Dielectric::new(1.5));
        let mat_right = Arc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

        world.push(Sphere::new(
            Vector3::new(0., -100.5, -1.),
            100.0,
            mat_ground,
        ));
        world.push(Sphere::new(Vector3::new(0., 0., -1.), 0.5, mat_center));
        world.push(Sphere::new(
            Vector3::new(-1., 0., -1.),
            0.5,
            mat_left.clone(),
        ));
        world.push(Sphere::new(Vector3::new(-1., 0., -1.), -0.45, mat_left));
        world.push(Sphere::new(Vector3::new(1., 0., -1.), 0.5, mat_right));

        world
    }

    pub fn example_many() -> Self {
        let mut rng = rand::thread_rng();
        let mut world = HittableList::default();
        let ground_material = Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));

        world.push(Sphere::new(
            Vector3::new(0., -1000., 0.),
            1000.,
            ground_material,
        ));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rng.gen::<f32>();
                let center = Vector3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );

                if (center - Vector3::new(4., 0.2, 0.)).magnitude() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = random_color(0., 1.).component_mul(&random_color(0., 1.));
                        let mat = Arc::new(Lambertian::new(albedo));
                        world.push(Sphere::new(center, 0.2, mat));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = random_color(0.5, 1.);
                        let fuzziness = rng.gen_range(0.0..=0.5);
                        let mat = Arc::new(Metal::new(albedo, fuzziness));
                        world.push(Sphere::new(center, 0.2, mat));
                    } else {
                        let mat = Arc::new(Dielectric::new(1.5));
                        world.push(Sphere::new(center, 0.2, mat));
                    }
                }
            }
        }

        let mat1 = Arc::new(Dielectric::new(1.5));
        world.push(Sphere::new(Vector3::new(0., 1., 0.), 1., mat1));

        let mat2 = Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
        world.push(Sphere::new(Vector3::new(-4., 1., 0.), 1., mat2));

        let mat3 = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.));
        world.push(Sphere::new(Vector3::new(4., 1., 0.), 1., mat3));

        world
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest = t_max;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, closest) {
                closest = hit.t;
                record = Some(hit);
            }
        }

        record
    }
}
