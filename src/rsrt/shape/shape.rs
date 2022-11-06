use enum_dispatch::enum_dispatch;

use super::plane::Plane;
use super::sphere::Sphere;
use crate::{material::hit::Hit, ray::Ray};

#[enum_dispatch]
pub enum ShapeType {
    Sphere,
    Plane,
}

#[enum_dispatch(ShapeType)]
pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}
