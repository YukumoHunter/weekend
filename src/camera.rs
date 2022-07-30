use super::{material::random_in_unit_disk, ray::Ray};
use nalgebra::Vector3;

pub struct Camera {
    pub pos: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub w: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub lens_radius: f32,
}

pub struct CameraBuilder {
    pub pos: Vector3<f32>,
    pub lookat: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov_vert_deg: f32,
    pub aspect_ratio: f32,
    pub aperture: f32,
    pub focus_dist: f32,
}

impl CameraBuilder {
    pub fn pos(mut self, pos: Vector3<f32>) -> Self {
        self.pos = pos;
        self
    }

    pub fn lookat(mut self, lookat: Vector3<f32>) -> Self {
        self.lookat = lookat;
        self
    }

    pub fn up(mut self, up: Vector3<f32>) -> Self {
        self.up = up;
        self
    }

    pub fn fov_vert_deg(mut self, fov_vert_deg: f32) -> Self {
        self.fov_vert_deg = fov_vert_deg;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn aperture(mut self, aperture: f32) -> Self {
        self.aperture = aperture;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f32) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn build(self) -> Camera {
        Camera::create(self)
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            pos: Vector3::new(0., 0., 0.),
            lookat: Vector3::new(0., 0., -1.),
            up: Vector3::new(0., 1., 0.),
            fov_vert_deg: 90.,
            aspect_ratio: 16. / 9.,
            aperture: 0.1,
            focus_dist: 10.,
        }
    }
}

fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn create(builder: CameraBuilder) -> Self {
        let theta = deg_to_rad(builder.fov_vert_deg);
        let h = (theta / 2.).tan();

        let viewport_height = 2. * h;
        let viewport_width = builder.aspect_ratio * viewport_height;

        // let focal_length = 1.0;
        let pos = builder.pos;

        let w = (pos - builder.lookat).normalize();
        let u = (builder.up.cross(&w)).normalize();
        let v = w.cross(&u);

        let horizontal = builder.focus_dist * viewport_width * u;
        let vertical = builder.focus_dist * viewport_height * v;
        let lower_left_corner = pos - horizontal / 2. - vertical / 2. - builder.focus_dist * w;

        Camera {
            pos,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius: builder.aperture / 2.,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let random = self.lens_radius * random_in_unit_disk();
        let offset = self.u * random.x + self.v * random.y;

        Ray::new(
            self.pos + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.pos - offset,
        )
    }
}
