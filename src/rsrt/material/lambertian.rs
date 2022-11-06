use crate::{ray::Ray, vec3::Vec3};

use super::{
    hit::Hit,
    material::{Material, Scatter},
};

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let direction = (Vec3::random_inside_unit() + hit.normal).normalize();
        let ray = Ray::new(hit.position, direction);
        Some(Scatter::new(self.albedo, ray))
    }
}
