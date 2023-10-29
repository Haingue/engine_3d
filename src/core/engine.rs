use super::math::{Vec2, Triangle};

#[derive(Debug)]
pub struct Engine {
  pub width: usize,
  pub height: usize,
  pub pixel_buffer_size: usize,
  pub pixel_buffer: Vec<char>
}

impl ToString for Engine {
    fn to_string(&self) -> String {
        format!("Engine [width={}, height={}, pixel_buffer_size={}]", self.width, self.height, self.pixel_buffer_size)
    }
}

impl Engine {
  pub fn new (width: usize, height: usize) -> Engine {
    if width < 1 || height < 1 {
      panic!("The width or height must be upper than 0");
    }
    Engine {
      width: width.clone(),
      height: height.clone(),
      pixel_buffer_size: width * height,
      pixel_buffer: vec![' ' ; width * height]
    }
  }
  
  pub fn draw (&self) {
    println!("{}", String::from_iter(&self.pixel_buffer));
  }

  pub fn clear (&mut self, char: char) {
    for idx in 0..self.pixel_buffer_size {
      (self.pixel_buffer)[idx] = char;
    }
  }

  pub fn put_pixel (&mut self, pixel: &Vec2, char: char) {
    let px = pixel.x;
    let py = pixel.y;
    if 0 <= px && px < self.width && 0 <= py && py < self.height {
      (self.pixel_buffer)[(py * self.width) + px] = char;
    }
  }

  pub fn put_triangle (&mut self, tri: &Triangle, char: char) {
    fn eq (p:Vec2, a:Vec2, b:Vec2) -> usize {
      a.x.wrapping_sub(p.x).wrapping_mul(b.y.wrapping_sub(p.y)).wrapping_sub(a.y.wrapping_sub(p.y).wrapping_mul(b.x.wrapping_sub(p.x)))
    }
    let x_set = vec![tri.p1.x, tri.p2.x, tri.p3.x];
    let xmin:&usize = x_set.iter().min().unwrap();
    let xmax:&usize = x_set.iter().max().unwrap();
    let y_set = vec![tri.p1.y, tri.p2.y, tri.p3.y];
    let ymin:&usize = y_set.iter().min().unwrap();
    let ymax:&usize = y_set.iter().max().unwrap();
    for y in *ymin..*ymax {
      for x in *xmin..*xmax {
        if x < self.width {
          let pos = Vec2::new(x, y);
          let w1:isize = eq(pos, tri.p3, tri.p1) as isize;
          let w2:isize = eq(pos, tri.p1, tri.p2) as isize;
          let w3:isize = eq(pos, tri.p2, tri.p3) as isize;
          if (w1 >= 0 && w2 >= 0 && w3 >= 0) || (-w1 >= 0 && -w2 >= 0 && -w3 >= 0) {
            self.put_pixel(&pos, char);
          }
        }
      }
    }
  }
}