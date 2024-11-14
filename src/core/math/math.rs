use super::vector::Vec3;

/*
 * Produit scalaire
 */
pub fn dot (v1:  Vec3, v2: Vec3) -> f32 {
  v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

/*
 * Produit vectoriel
 */
pub fn cross_prod (v1: Vec3, v2: Vec3) -> Vec3 {
  Vec3 {
    x: v1.y * v2.z - v1.z * v2.y,
    y: v1.z * v2.x - v1.x * v2.z,
    z: v1.x * v2.y - v1.y * v2.x,
  }
}

pub fn line_plane_intersection (normal_plane: Vec3, normal_point: Vec3, v1: Vec3, v2: Vec3) -> Vec3 {
  let mut u: Vec3 = v2 - v1;
  let dot_p: f32 = dot (normal_plane, u);
  if dot_p.abs() < 0.00001 {
    return Vec3::new(0.0, 0.0, 0.0);
  }
  let w: Vec3 = v1 - normal_point;
  let si: f32 = - dot(normal_plane, w) / dot_p;
  u = si*u;
  v1 + u
}
