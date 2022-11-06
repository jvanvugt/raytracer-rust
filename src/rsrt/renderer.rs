use image::{ImageBuffer, RgbImage};

use crate::{camera::Camera, vec3::Vec3, world::World};

pub struct Renderer {
    pub num_samples: u32,
    pub max_bounces: u32,
}

fn map_byte(s: f32) -> u8 {
    assert!(s >= 0.0 && s <= 1.0, "{}", s);
    return (s * 255.999) as u8;
}

fn to_color(v: Vec3) -> image::Rgb<u8> {
    image::Rgb([map_byte(v.x), map_byte(v.y), map_byte(v.z)])
}

impl Renderer {
    pub fn new(num_samples: u32, max_bounces: u32) -> Self {
        Self {
            num_samples,
            max_bounces,
        }
    }

    fn process_pixel(&self, x: u32, y: u32, camera: &Camera, world: &World) -> Vec3 {
        let mut color = Vec3::ZERO;
        for _ in 0..self.num_samples {
            let ray = camera.make_ray(
                x as f32 + rand::random::<f32>() - 0.5,
                y as f32 + rand::random::<f32>() - 0.5,
            );
            color = color + world.cast_ray(&ray, 0, self.max_bounces);
        }
        color = color / self.num_samples as f32;
        color
    }

    pub fn render(&self, camera: &Camera, world: &World, width: u32, height: u32) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let color = self.process_pixel(x, y, &camera, &world);
                img.put_pixel(x, height - y - 1, to_color(color));
            }
        }
        img
    }
}
