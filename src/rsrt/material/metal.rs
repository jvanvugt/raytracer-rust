use crate::{math::reflect, ray::Ray, vec3::Vec3};

use super::{
    hit::Hit,
    material::{Material, Scatter},
};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let direction = reflect(&ray.direction, &hit.normal);
        let fuzzed_dir = (direction + Vec3::random_inside_unit() * self.fuzz).normalize();
        if fuzzed_dir.dot(&hit.normal) > 0.0 {
            let bounce = Ray::new(hit.position, fuzzed_dir);
            Some(Scatter::new(self.albedo, bounce))
        } else {
            None
        }
    }
}
