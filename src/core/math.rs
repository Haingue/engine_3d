#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
  pub x: usize,
  pub y: usize
}

impl Vec2 {
  pub fn new (x: usize, y:usize) -> Vec2{
    Vec2 { x, y }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
  pub p1: Vec2,
  pub p2: Vec2,
  pub p3: Vec2
}

impl Triangle {
  pub fn new (p1: Vec2, p2: Vec2, p3: Vec2) -> Triangle {
    Triangle { p1, p2, p3 }
  }
  pub fn update_p1 (&mut self, x: usize, y: usize) {
    self.p1.x = x;
    self.p1.y = y;
  }
  pub fn update_p2 (&mut self, x: usize, y: usize) {
    self.p2.x = x;
    self.p2.y = y;
  }
  pub fn update_p3 (&mut self, x: usize, y: usize) {
    self.p3.x = x;
    self.p3.y = y;
  }
}