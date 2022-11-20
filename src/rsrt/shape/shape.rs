use enum_dispatch::enum_dispatch;

use super::plane::{Plane, PlaneWithPoint};
use super::sphere::Sphere;
use super::triangle::Triangle;
use crate::{material::hit::Hit, ray::Ray};

#[enum_dispatch]
pub enum ShapeType {
    Sphere,
    Plane,
    PlaneWithPoint,
    Triangle,
}

#[enum_dispatch(ShapeType)]
pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}
