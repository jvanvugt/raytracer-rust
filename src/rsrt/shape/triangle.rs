use crate::{
    material::{hit::Hit, material::MaterialType},
    math::in01,
    ray::Ray,
    vec3::Vec3,
};

use super::{plane::PlaneWithPoint, shape::Shape};

pub struct Triangle {
    // Triangle in 3D space. Vertices are counter-clockwise
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
    pub material: MaterialType,
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, material: MaterialType) -> Self {
        Self {
            v1,
            v2,
            v3,
            material,
        }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let relv2 = self.v2 - self.v1;
        let relv3 = self.v3 - self.v1;
        let normal = relv2.cross(&relv3);
        let plane = PlaneWithPoint::new(normal.normalize(), self.v1, self.material);
        let hit_opt = plane.intersect(ray);
        if let Some(hit) = hit_opt {
            let normal_squared_length = normal.sq_length();
            let gamma = relv2.cross(&hit.position).dot(&normal) / normal_squared_length;
            let beta = (hit.position - self.v1).cross(&relv3).dot(&normal) / normal_squared_length;
            let alpha = 1.0 - gamma - beta;
            if in01(alpha) && in01(beta) && in01(gamma) {
                Some(Hit::from_ray(
                    hit.t,
                    ray,
                    normal.normalize(),
                    &self.material,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
