use crate::moving::{Timespan, Trajectory};

use super::{hittable::HittableList, material::*, sphere::*, texture::Checker};
use nalgebra::Vector3;
use rand::Rng;
use std::sync::Arc;

pub fn weekend() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::default();
    let ground_material = Lambertian::from_color(Vector3::new(0.5, 0.5, 0.5));

    world.push(Box::new(Sphere::new(
        Vector3::new(0., -1000., 0.),
        1000.,
        Arc::new(ground_material),
    )));

    for a in -10..=10 {
        for b in -10..=10 {
            let choose_mat = rng.gen::<f32>();
            let center = Vector3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_color(0., 1.).component_mul(&random_color(0., 1.));
                    let mat = Lambertian::from_color(albedo);
                    world.push(Box::new(Sphere::new(center, 0.2, Arc::new(mat))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_color(0.5, 1.);
                    let fuzziness = rng.gen_range(0.0..=0.5);
                    let mat = Metal::from_color(albedo, fuzziness);
                    world.push(Box::new(Sphere::new(center, 0.2, Arc::new(mat))));
                } else {
                    // glass
                    let mat = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, Arc::new(mat))));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Vector3::new(0., 1., 0.),
        1.,
        Arc::new(mat1),
    )));

    let mat2 = Lambertian::from_color(Vector3::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Vector3::new(-4., 1., 0.),
        1.,
        Arc::new(mat2),
    )));

    let mat3 = Metal::from_color(Vector3::new(0.7, 0.6, 0.5), 0.);
    world.push(Box::new(Sphere::new(
        Vector3::new(4., 1., 0.05),
        1.,
        Arc::new(mat3),
    )));

    world
}

pub fn glass_balls() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Arc::new(Checker::from_colors(
        Vector3::new(0., 0., 0.),
        Vector3::new(1., 1., 1.),
    )));

    world.push(Box::new(Sphere::new(
        Vector3::new(0., -1000., 0.),
        1000.,
        Arc::new(ground_material),
    )));

    let trajectory = Trajectory::new(
        Vector3::new(-1., 1., 0.),
        Vector3::new(1., 1., 0.),
        Timespan::new(0., 10.),
    );

    let mat = Arc::new(Dielectric::new(1.5));
    for x in -5..=5 {
        for y in 0..=5 {
            for z in -5..=5 {
                world.push(Box::new(Sphere::new(
                    Vector3::new(x as f32, 0.4 + 0.8 * y as f32, z as f32),
                    0.4,
                    mat.clone(),
                )));
                world.push(Box::new(MovingSphere::new(trajectory, 0.35, mat.clone())));
            }
        }
    }

    world
}
