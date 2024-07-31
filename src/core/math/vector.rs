use std::ops;

use crate::core::engine::Engine;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32
}

impl ops::Mul<f32> for Vec2 {
  type Output = Vec2;
  fn mul(self, rhs: f32) -> Self::Output {
    Vec2 {
      x: self.x * rhs,
      y: self.y * rhs
    }
  }
}
impl ops::Mul<Vec2> for f32 {
  type Output = Vec2;
  fn mul(self, v_rhs: Vec2) -> Self::Output {
    Vec2 {
      x: self * v_rhs.x,
      y: self * v_rhs.y
    }
  }
}
impl ops::Div<f32> for Vec2 {
  type Output = Vec2;
  fn div(self, rhs: f32) -> Self::Output {
    Vec2 {
      x: self.x / rhs,
      y: self.y / rhs
    }
  }
}
impl ops::Add<Vec2> for Vec2 {
  type Output = Vec2;
  fn add(self, v_rhs: Vec2) -> Vec2 {
    Vec2 {
      x: self.x + v_rhs.x,
      y: self.y + v_rhs.y
    }
  }
}

impl Vec2 {
  pub fn new (x: f32, y:f32) -> Vec2 {
    Vec2 { x, y }
  }
  pub fn toScreen (&mut self, engine: &Engine) -> Vec2 {
    let height = engine.height as f32;
   let width = engine.width as f32;
    Vec2 {
      x: (height/width * self.x + 1.0) * (width as f32) / 2.0,
      y: (-self.y + 1.0) * (height as f32) / 2.0
     }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}
impl ops::Mul<f32> for Vec3 {
  type Output = Vec3;
  fn mul(self, rhs: f32) -> Self::Output {
    Vec3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs
    }
  }
}
impl ops::Mul<Vec3> for f32 {
  type Output = Vec3;
  fn mul(self, v_rhs: Vec3) -> Self::Output {
    Vec3 {
      x: self * v_rhs.x,
      y: self * v_rhs.y,
      z: self * v_rhs.z
    }
  }
}
impl ops::Div<f32> for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: f32) -> Self::Output {
    Vec3 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs
    }
  }
}
impl ops::Add<Vec3> for Vec3 {
  type Output = Vec3;
  fn add(self, v_rhs: Vec3) -> Self::Output {
    Vec3 {
      x: self.x + v_rhs.x,
      y: self.y + v_rhs.y,
      z: self.z + v_rhs.z
    }
  }
}
impl ops::AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, v_rhs: Vec3) {
    *self = Vec3 {
      x: self.x + v_rhs.x,
      y: self.y + v_rhs.y,
      z: self.z + v_rhs.z
    };
  }
}
impl ops::Sub<Vec3> for Vec3 {
  type Output = Vec3;
  fn sub(self, v_rhs: Vec3) -> Self::Output {
    Vec3 {
      x: self.x - v_rhs.x,
      y: self.y - v_rhs.y,
      z: self.z - v_rhs.z
    }
  }
}
impl ops::SubAssign<Vec3> for Vec3 {
  fn sub_assign(&mut self, v_rhs: Vec3) {
    *self = Vec3 {
      x: self.x - v_rhs.x,
      y: self.y - v_rhs.y,
      z: self.z - v_rhs.z
    };
  }
}

impl Vec3 {
  pub fn new (x: f32, y:f32, z:f32) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn projection (&mut self, focal_length: f32) -> Vec2 {
    return focal_length * Vec2 {
      x: self.x,
      y: self.y
    } / self.z
  }
  
  pub fn rotation_x (&mut self, pitch:f32) -> Vec3 {
    let y = f32::cos(pitch) * self.y - f32::sin(pitch) * self.z;
    let z = f32::sin(pitch) * self.y + f32::cos(pitch) * self.z;
    Vec3 {x: self.x, y, z}
  }

  pub fn rotation_y (&mut self, yaw:f32) -> Vec3 {
    let x = f32::cos(yaw) * self.x + f32::sin(yaw) * self.z;
    let z = -f32::sin(yaw) * self.x + f32::cos(yaw) * self.z;
    Vec3 {x, y: self.y, z}
  }
}