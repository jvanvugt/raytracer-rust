use crate::{
    material::{hit::Hit, material::MaterialType},
    ray::Ray,
    vec3::Vec3,
};

use super::shape::Shape;

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub material: MaterialType,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, material: MaterialType) -> Self {
        Self {
            position,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let a = ray.direction.sq_length();
        let rel_pos = ray.origin - self.position;
        let half_b = ray.direction.dot(&rel_pos);
        let c = rel_pos.sq_length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let d_sqrt = discriminant.sqrt();
        let mut t = (-half_b - d_sqrt) / a;
        if t < 0.0 {
            t = (-half_b + d_sqrt) / a;
        }
        if t <= 1e-3 {
            return None;
        }
        let normal = ((ray.at(t) - self.position) / self.radius).normalize();
        Some(Hit::from_ray(t, ray, normal, &self.material))
    }
}
