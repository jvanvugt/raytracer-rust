use crate::vec3::Vec3;

pub fn reflect(incoming: &Vec3, normal: &Vec3) -> Vec3 {
    *incoming - *normal * normal.dot(incoming) * 2.0
}
