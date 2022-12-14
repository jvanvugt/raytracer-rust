use enum_dispatch::enum_dispatch;

use crate::{ray::Ray, vec3::Vec3};

use super::dielectric::Dielectric;
use super::hit::Hit;
use super::lambertian::Lambertian;
use super::metal::Metal;

pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vec3, ray: Ray) -> Self {
        Self { attenuation, ray }
    }
}

#[derive(Clone, Copy)]
#[enum_dispatch]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

#[enum_dispatch(MaterialType)]
pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
