pub mod camera;
pub mod hittable;
pub mod image;
pub mod material;
pub mod ray;
pub mod sphere;

use camera::Camera;
use hittable::HittableList;
use image::{generate_ppm, RayTraceOptions};
use nalgebra::Vector3;
use std::fs::write;

fn main() {
    let aspect_ratio = 3. / 2.;
    let camera = Camera::builder()
        .pos(Vector3::new(13., 2., 3.))
        .lookat(Vector3::new(0., 0., 0.))
        .up(Vector3::new(0., 1., 0.))
        .fov_vert_deg(20.)
        .aspect_ratio(aspect_ratio)
        .aperture(0.1)
        .focus_dist(10.)
        .build();

    let world = HittableList::example_many();

    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    let samples_per_pixel = 1000;
    let max_depth = 50;

    let opts = RayTraceOptions::new(samples_per_pixel, max_depth);

    let image = generate_ppm([image_width, image_height], &camera, &world, opts);

    write("./image.ppm", image).unwrap();
}
