use super::{
    camera::Camera,
    hittable::{Hittable, HittableList},
    ray::Ray,
};
use indicatif::ParallelProgressIterator;
use nalgebra::Vector3;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fmt::Write;

#[derive(Clone, Copy)]
pub struct RayTraceOptions {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl RayTraceOptions {
    pub fn new(samples_per_pixel: u32, max_depth: u32) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
        }
    }
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Vector3<f32> {
    if depth == 0 {
        return Vector3::<f32>::zeros();
    }

    if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
        if let Some(scatter) = hit.material.scatter(ray, &hit) {
            return ray_color(&scatter.ray, world, depth - 1).component_mul(&scatter.attenuation);
        }
        return Vector3::<f32>::zeros();
    };

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

pub fn generate_ppm(
    resolution: [u32; 2],
    camera: &Camera,
    world: &HittableList,
    rt_options: RayTraceOptions,
) -> String {
    let [image_width, image_height] = resolution;

    let mut output = format!("P3\n{image_width} {image_height}\n255\n");

    let image = (0..image_height)
        .into_par_iter()
        .rev()
        .progress()
        .flat_map(|y| {
            (0..image_width)
                .flat_map(|x| {
                    let color: Vector3<f32> = (0..rt_options.samples_per_pixel)
                        .map(|_| {
                            let mut rng = rand::thread_rng();

                            let u = (x as f32 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f32;
                            let v =
                                (y as f32 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f32;
                            let ray = camera.get_ray(u, v);

                            ray_color(&ray, world, rt_options.max_depth)
                        })
                        .sum();

                    color
                        .iter()
                        .map(|c| {
                            // average rays, perform gamma correction and scaling
                            ((c / rt_options.samples_per_pixel as f32).sqrt() * 255.99) as u8
                        })
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for scanline in image.chunks(image_width as usize) {
        for pixel in scanline.chunks(3) {
            write!(output, "{} {} {} ", pixel[0], pixel[1], pixel[2]).unwrap();
        }
        output.push('\n');
    }

    output
}
