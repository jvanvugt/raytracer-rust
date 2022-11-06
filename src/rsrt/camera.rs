use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pub position: Vec3,
    bottom_left: Vec3,
    pixel_step_x: Vec3,
    pixel_step_y: Vec3,
}

impl Camera {
    pub fn new(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        img_height: u32,
        img_width: u32,
    ) -> Self {
        let direction = (target - position).normalize();
        let horizontal = up.normalize().cross(&direction).normalize();
        let vertical = direction.cross(&horizontal).normalize();
        let half_width = (fov.to_radians() / 2.0).tan();
        let half_height = half_width * img_height as f32 / img_width as f32;
        let pixel_step_x = horizontal * (2.0 * half_width / (img_width - 1) as f32);
        let pixel_step_y = vertical * (2.0 * half_height / (img_height - 1) as f32);
        let bottom_left = direction - horizontal * half_width - vertical * half_height;
        Camera {
            position,
            bottom_left,
            pixel_step_x,
            pixel_step_y,
        }
    }

    pub fn make_ray(&self, x: f32, y: f32) -> Ray {
        let direction = self.bottom_left + self.pixel_step_x * x + self.pixel_step_y * y;
        Ray::new(self.position, direction.normalize())
    }
}
