use std::cell::RefCell;

use crate::vec3::Vec3;
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
    *incoming - *normal * normal.dot(incoming) * 2.0
}

thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(0));
}

pub fn rand01() -> f32 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

pub fn rand_m1_1() -> f32 {
    (rand01() - 0.5) * 2.0
}
