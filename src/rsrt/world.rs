use crate::material::dielectric::Dielectric;
use crate::material::material::Material;
use crate::material::{lambertian::Lambertian, metal::Metal};
use crate::shape::shape::{Shape, ShapeType};
use crate::shape::{plane::Plane, sphere::Sphere};
use crate::{ray::Ray, vec3::Vec3};

pub struct World {
    pub objects: Vec<ShapeType>,
}

impl World {
    pub fn new(objects: Vec<ShapeType>) -> Self {
        Self { objects }
    }

    pub fn cast_ray(&self, ray: &Ray, bounced: u32, max_bounces: u32) -> (Vec3, u32) {
        if bounced >= max_bounces {
            return (Vec3::ZERO, bounced);
        }
        let closest_hit = self
            .objects
            .iter()
            .filter_map(|s| s.intersect(ray))
            .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap_or(std::cmp::Ordering::Equal));
        if let Some(hit) = closest_hit {
            let scatter = hit.material.scatter(ray, &hit);
            if let Some(scatter) = scatter {
                let (color, bounced) = self.cast_ray(&scatter.ray, bounced + 1, max_bounces);
                (color * scatter.attenuation, bounced)
            } else {
                (Vec3::ZERO, bounced)
            }
        } else {
            let y = (ray.direction.y + 1.0) / 2.0;
            let res = Vec3::new(0.6, 0.6, 1.0) * y + Vec3::ONE * (1.0 - y);
            (res.clamp(&Vec3::ZERO, &Vec3::ONE), bounced)
        }
    }

    pub fn make_metallic_example() -> Self {
        Self::new(vec![
            Plane::new(
                Vec3::new(0.0, 1.0, 0.0),
                -1.0,
                Lambertian::new(Vec3::new(140.0 / 255.0, 245.0 / 255.0, 98.0 / 255.0)).into(),
            )
            .into(),
            // Sphere::new(
            //     Vec3::new(-2.0, 0.0, 2.0),
            //     1.0,
            //     Metal::new(Vec3::ONE, 0.2).into(),
            // )
            // .into(),
            // Sphere::new(
            //     Vec3::new(0.0, 0.0, 2.0),
            //     1.0,
            //     Lambertian::new(Vec3::new(1.0, 200.0 / 255.0, 210.0 / 255.0)).into(),
            // )
            // .into(),
            // Sphere::new(
            //     Vec3::new(2.0, 0.0, 2.0),
            //     1.0,
            //     Metal::new(Vec3::new(0.8, 0.75, 1.0), 0.0).into(),
            // )
            // .into(),
            Sphere::new(Vec3::new(0.0, 0.0, 2.0), 1.0, Dielectric::new(1.5).into()).into(),
        ])
    }
}
