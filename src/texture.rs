use nalgebra::Vector3;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
pub struct Uv {
    pub u: f32,
    pub v: f32,
}

impl Uv {
    pub fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }
}

pub trait Texture: Sync + Send + Debug {
    fn value(&self, uv: &Uv, point: &Vector3<f32>) -> Vector3<f32>;
}

#[derive(Debug)]
pub struct SolidColor {
    pub color_value: Vector3<f32>,
}

impl SolidColor {
    pub fn new(color_value: Vector3<f32>) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: &Uv, _point: &Vector3<f32>) -> Vector3<f32> {
        self.color_value
    }
}

#[derive(Debug)]
pub struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn from_colors(odd: Vector3<f32>, even: Vector3<f32>) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, uv: &Uv, point: &Vector3<f32>) -> Vector3<f32> {
        let sines = (10. * point).map(|x| x.sin()).product();
        if sines < 0. {
            self.odd.value(uv, point)
        } else {
            self.even.value(uv, point)
        }
    }
}
