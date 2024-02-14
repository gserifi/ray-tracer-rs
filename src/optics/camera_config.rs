use crate::utils::{Point3, Vec3};

pub struct RenderOutputConfig {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl Default for RenderOutputConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 100,
            samples_per_pixel: 100,
            max_depth: 50,
        }
    }
}

pub struct ViewportConfig {
    pub vertical_fov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Vec3,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            vertical_fov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

pub struct LensConfig {
    pub depth_of_field_angle: f64,
    pub focus_dist: f64,
}

impl Default for LensConfig {
    fn default() -> Self {
        Self {
            depth_of_field_angle: 0.0,
            focus_dist: 1.0,
        }
    }
}
