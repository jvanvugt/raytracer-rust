use crate::{
    material::{hit::Hit, material::MaterialType},
    ray::Ray,
    vec3::Vec3,
};

use super::shape::Shape;

pub struct Plane {
    pub normal: Vec3,
    pub along: f32,
    pub material: MaterialType,
    p: f32,
}

impl Plane {
    pub fn new(normal: Vec3, along: f32, material: MaterialType) -> Self {
        Self {
            normal,
            along,
            material,
            p: (normal * along).dot(&normal),
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-10 {
            return None;
        }
        let t = (self.p - self.normal.dot(&ray.origin)) / denom;
        if t < 1e-10 {
            None
        } else {
            Some(Hit::from_ray(t, ray, self.normal, &self.material))
        }
    }
}
