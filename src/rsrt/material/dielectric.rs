use crate::{
    math::{near_equal, rand01, reflect, refract, schlick},
    ray::Ray,
    vec3::Vec3,
};

use super::{
    hit::Hit,
    material::{Material, Scatter},
};

pub struct Dielectric {
    reflection_index: f32,
}

impl Dielectric {
    pub fn new(reflection_index: f32) -> Self {
        Self { reflection_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let d = ray.direction.dot(&hit.normal);
        let (outward_normal, ni_over_nt, cosine) = if d > 0.0 {
            (
                -hit.normal,
                self.reflection_index,
                self.reflection_index * d,
            )
        } else {
            (hit.normal, 1.0 / self.reflection_index, -d)
        };
        assert!(near_equal(hit.normal.sq_length(), 1.0, 1e-3));
        assert!(near_equal(ray.direction.sq_length(), 1.0, 1e-3));
        let refracted = refract(&ray.direction, &outward_normal, ni_over_nt);
        let direction = if refracted.is_some() && schlick(cosine, self.reflection_index) <= rand01()
        {
            refracted.unwrap()
        } else {
            reflect(&ray.direction, &hit.normal)
        };
        Some(Scatter::new(Vec3::ONE, Ray::new(hit.position, direction)))
    }
}
