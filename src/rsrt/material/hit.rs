use crate::{ray::Ray, vec3::Vec3};

use super::material::MaterialType;

pub struct Hit<'a> {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub material: &'a MaterialType,
}

impl<'a> Hit<'a> {
    pub fn new(t: f32, position: Vec3, normal: Vec3, material: &'a MaterialType) -> Self {
        Self {
            t,
            position,
            normal,
            material,
        }
    }

    pub fn from_ray(t: f32, ray: &Ray, normal: Vec3, material: &'a MaterialType) -> Self {
        Self::new(t, ray.at(t), normal, &material)
    }
}
