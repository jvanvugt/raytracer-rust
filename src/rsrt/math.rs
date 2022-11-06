use std::cell::RefCell;

use crate::vec3::Vec3;
use rand::{Rng, SeedableRng};

pub fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
    *incoming - *normal * normal.dot(incoming) * 2.0
}

pub fn refract(incoming: &Vec3, normal: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let dt = incoming.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refraction =
            ((*incoming - (*normal * dt)) * ni_over_nt - *normal * discriminant.sqrt()).normalize();
        Some(refraction)
    } else {
        None
    }
}

pub fn schlick(cosine: f32, reflection_index: f32) -> f32 {
    let r = (1.0 - reflection_index) / (1.0 + reflection_index);
    let r2 = r * r;
    let base = 1.0 - cosine;
    r2 + (1.0 - r2) * base * base * base * base * base
}

thread_local! {
    pub static RNG: RefCell<rand_xoshiro::Xoroshiro128Plus> = RefCell::new(rand_xoshiro::Xoroshiro128Plus::seed_from_u64(0));
}

pub fn rand01() -> f32 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

pub fn rand_m1_1() -> f32 {
    (rand01() - 0.5) * 2.0
}
