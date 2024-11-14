use crate::core::engine::Engine;

use super::vector::{Vec2, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Triangle2D {
  pub v1: Vec2,
  pub v2: Vec2,
  pub v3: Vec2
}

impl Triangle2D {
  pub fn new (v1: Vec2, v2: Vec2, v3: Vec2) -> Triangle2D {
    Triangle2D { v1, v2, v3 }
  }
  pub fn update_v1 (&mut self, x: f32, y: f32) {
    self.v1.x = x;
    self.v1.y = y;
  }
  pub fn update_v2 (&mut self, x: f32, y: f32) {
    self.v2.x = x;
    self.v2.y = y;
  }
  pub fn update_v3 (&mut self, x: f32, y: f32) {
    self.v3.x = x;
    self.v3.y = y;
  }
  pub fn to_screen (&mut self, engine: &Engine) -> Triangle2D {
    Triangle2D {
      v1: self.v1.to_screen(engine),
      v2: self.v2.to_screen(engine),
      v3: self.v3.to_screen(engine)
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle3D {
  pub v1: Vec3,
  pub v2: Vec3,
  pub v3: Vec3
}
impl Triangle3D {
  pub fn new (v1: Vec3, v2: Vec3, v3: Vec3) -> Triangle3D {
    Triangle3D { v1, v2, v3 }
  }
  pub fn projection (&mut self, focal_length: f32) -> Triangle2D {
    Triangle2D {
      v1: self.v1.projection(focal_length),
      v2: self.v2.projection(focal_length),
      v3: self.v3.projection(focal_length)
    }
  }
  pub fn translate (&mut self, v: Vec3) -> Triangle3D {
    Triangle3D {
      v1: self.v1 + v,
      v2: self.v2 + v,
      v3: self.v3 + v
    }
  }
  pub fn rotation_x (&mut self, pitch:f32) -> Triangle3D {
    Triangle3D {
      v1: self.v1.rotation_x(pitch),
      v2: self.v2.rotation_x(pitch),
      v3: self.v3.rotation_x(pitch)
    }
  }
  pub fn rotation_y (&mut self, yaw:f32) -> Triangle3D {
    Triangle3D {
      v1: self.v1.rotation_y(yaw),
      v2: self.v2.rotation_y(yaw),
      v3: self.v3.rotation_y(yaw)
    }
  }
}
