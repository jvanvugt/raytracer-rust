use std::time::Instant;

use image::{ImageBuffer, RgbImage};

use crate::{camera::Camera, math::rand01, vec3::Vec3, world::World};

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

    fn process_pixel(&self, x: u32, y: u32, camera: &Camera, world: &World) -> (Vec3, u64) {
        let mut color = Vec3::ZERO;
        let mut num_rays = 0;
        for _ in 0..self.num_samples {
            let ray = camera.make_ray(x as f32 + rand01() - 0.5, y as f32 + rand01() - 0.5);
            let (ray_color, num_bounces) = world.cast_ray(&ray, 0, self.max_bounces);
            num_rays += num_bounces + 1;
            color = color + ray_color;
        }
        color = color / self.num_samples as f32;
        (color, num_rays.into())
    }

    pub fn render(&self, camera: &Camera, world: &World, width: u32, height: u32) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(width, height);
        let mut total_rays = 0;

        let start_time = Instant::now();
        for y in 0..height {
            for x in 0..width {
                let (color, n_rays) = self.process_pixel(x, y, &camera, &world);
                img.put_pixel(x, height - y - 1, to_color(color));
                total_rays += n_rays;
            }
        }
        let render_time = start_time.elapsed().as_secs_f64();
        println!(
            "Simulated {} rays in {}s. {:.0} rays/s",
            total_rays,
            render_time,
            total_rays as f64 / render_time
        );
        img
    }
}
