use super::vector::Vec3;

pub fn dot (v1:  Vec3, v2: Vec3) -> f32 {
  v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

pub fn line_plane_intersection (normal_plane: Vec3, normal_point: Vec3, v1: Vec3, v2: Vec3) -> Vec3{
  let u: Vec3 = v2 - v1;
  // let dot_p: Vec3 = dot (normal_plane, u);
  // if dot_p
  u
}