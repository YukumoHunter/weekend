use std::cmp::Ordering;

use super::{hittable::HitRecord, ray::Ray};
use nalgebra::{Vector2, Vector3};
use rand::Rng;

pub fn random_color(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();

    Vector3::new(
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
    )
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0..=1.) as f32,
            rng.gen_range(-1.0..=1.),
            rng.gen_range(-1.0..=1.),
        );
        if p.norm_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk() -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector2::new(rng.gen_range(-1.0..=1.) as f32, rng.gen_range(-1.0..=1.));
        if p.norm_squared() >= 1. {
            continue;
        }
        return p;
    }
}

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2. * v.dot(n) * n
}

fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, eta_i_over_eta_t: f32) -> Vector3<f32> {
    let cos_theta = -uv.dot(n).min(1.);
    let r_out_perp = eta_i_over_eta_t * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.magnitude_squared()).abs().sqrt()) * n;

    r_out_perp + r_out_parallel
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vector3<f32>,
}

impl Scatter {
    pub fn new(ray: Ray, attenuation: Vector3<f32>) -> Self {
        Self { ray, attenuation }
    }
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let scatter_dir = hit_record.normal + random_in_unit_sphere().normalize();
        let ray = Ray::new(hit_record.p, scatter_dir);

        // TODO: catch all-zero scatter position

        Some(Scatter::new(ray, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzziness: f32) -> Self {
        Self {
            albedo,
            fuzziness: fuzziness.min(1.).max(0.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&ray_in.direction.normalize(), &hit_record.normal);
        let ray = Ray::new(
            hit_record.p,
            reflected + self.fuzziness * random_in_unit_sphere().normalize(),
        );

        match ray.direction.dot(&hit_record.normal).partial_cmp(&0.) {
            Some(Ordering::Greater) => Some(Scatter::new(ray, self.albedo)),
            _ => None,
        }
    }
}

pub struct Dielectric {
    // index of reflection
    pub ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f32, ir: f32) -> f32 {
        // Schlick's approximation for reflectance.
        let mut r0 = (1. - ir) / (1. + ir);
        r0 *= r0;

        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut rng = rand::thread_rng();

        let attenuation = Vector3::new(1., 1., 1.);

        let (normal_outward_facing, refraction_ratio) =
            match ray_in.direction.dot(&hit_record.normal).partial_cmp(&0.0) {
                // normal points inside sphere
                Some(Ordering::Greater) => (-hit_record.normal, self.ir),
                // normal points outside sphere
                _ => (hit_record.normal, 1. / self.ir),
            };

        let unit_direction = ray_in.direction.normalize();

        let cos_theta = (-unit_direction).dot(&normal_outward_facing).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..=1.)
        {
            reflect(&unit_direction, &normal_outward_facing)
        } else {
            refract(&unit_direction, &normal_outward_facing, refraction_ratio)
        };

        let ray = Ray::new(hit_record.p, direction);

        Some(Scatter::new(ray, attenuation))
    }
}
