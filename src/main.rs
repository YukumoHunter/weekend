pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod image;
pub mod material;
pub mod moving;
pub mod ray;
pub mod scenes;
pub mod sphere;
pub mod texture;

use bvh::BvhTree;
use camera::Camera;
use image::{render_ppm, RayTraceOptions};
use moving::Timespan;
use nalgebra::Vector3;
use std::fs;

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_height = 720;
    let image_width = (image_height as f32 * aspect_ratio) as u32;

    let camera = Camera::builder()
        .pos(Vector3::new(13., 2., 3.))
        .lookat(Vector3::new(0., 0., 0.))
        .up(Vector3::new(0., 1., 0.))
        .fov_vert_deg(20.)
        .aspect_ratio(aspect_ratio)
        .aperture(0.1)
        .focus_dist(10.)
        .build();

    let world = scenes::weekend();
    let bvh = BvhTree::new(world.raw, Timespan::new(0., 1.));

    let opts = RayTraceOptions::new(100, 10);
    let image = render_ppm([image_width, image_height], &camera, &bvh, &opts);

    fs::write("./image.ppm", image).unwrap();
}
