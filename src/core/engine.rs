use std::{io::{self}, ops::{Add, AddAssign}};

use crossterm::event::{read, Event, KeyCode, KeyModifiers};

use crate::core::math::math::{dot, line_plane_intersection};

use super::math::{vector::{Vec2, Vec3}, triangle::{Triangle2D, Triangle3D}};

#[derive(Debug)]
pub struct Engine {
  pub width: usize,
  pub height: usize,
  pub pixel_ratio: usize,
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
      pixel_ratio: 29/13,
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
    let width = self.width as f32;
    let height = self.height as f32;
    if 0.0 <= px && px < width && 0.0 <= py && py < height {
      let index: usize = ((py * width) + px) as usize;
      (self.pixel_buffer)[index] = char;
    }
  }

  pub fn put_triangle (&mut self, tri: &Triangle2D, char: char) {
    fn eq (p:Vec2, a:Vec2, b:Vec2) -> isize {
      ((a.x - p.x) * (b.y - p.y) - (a.y - p.y) * (b.x - p.x)) as isize
    }
    let width = self.width.try_into().unwrap();
    let height = self.height.try_into().unwrap();
    let x_set = vec![tri.v1.x as isize, tri.v2.x as isize, tri.v3.x as isize];
    let xmin:&isize = x_set.iter().min().unwrap();
    let xmax:&isize = x_set.iter().max().unwrap();
    let y_set = vec![tri.v1.y as isize, tri.v2.y as isize, tri.v3.y as isize];
    let ymin:&isize = y_set.iter().min().unwrap();
    let ymax:&isize = y_set.iter().max().unwrap();
    for y in *ymin..(*ymax+1.0 as isize) {
      if 0 <= y && y < height {
        for x in *xmin..(*xmax+1.0 as isize) {
          if 0 <= x && x < width {
            let pos = Vec2::new(x as f32, y as f32);
            let w1:isize = eq(pos, tri.v3, tri.v1) as isize;
            let w2:isize = eq(pos, tri.v1, tri.v2) as isize;
            let w3:isize = eq(pos, tri.v2, tri.v3) as isize;
            if (w1 >= 0 && w2 >= 0 && w3 >= 0) || (-w1 >= 0 && -w2 >= 0 && -w3 >= 0) {
              self.put_pixel(&pos, char);
            }
          }
        }
      }
    }
  }

  pub fn clip (&mut self, triangle: Triangle3D, cam: &Camera, normal_plane: Vec3) -> Vec<Triangle3D> {
    fn inZ (normal_plane: Vec3, normal_point: Vec3, triangle: Triangle3D) -> (Vec<Vec3>, Vec<Vec3>) {
      let mut out: Vec<Vec3> = vec![];
      let mut in_: Vec<Vec3> = vec![];
      let vert1 = dot(normal_point - triangle.v1, normal_plane);
      let vert2 = dot(normal_point - triangle.v2, normal_plane);
      let vert3 = dot(normal_point - triangle.v3, normal_plane);
      if vert1 > 0.0 {
        out.push(triangle.v1)
      } else {
        in_.push(triangle.v1)
      }
      if vert2 > 0.0 {
        out.push(triangle.v2)
      } else {
        in_.push(triangle.v2)
      }
      if vert3 > 0.0 {
        out.push(triangle.v3)
      } else {
        in_.push(triangle.v3)
      }
      return (out, in_);
    }
    let z_near: Vec3 = cam.position + 0.1 * normal_plane;
    let (out, in_) = inZ(normal_plane, z_near, triangle);
    if out.len() == 0 {
      return vec!(triangle);
    } else if out.len() == 3 {
        return vec![];
    } else if out.len() == 2 {
        let intersection0 = line_plane_intersection(normal_plane, z_near, triangle.v1, triangle.v2);
        // TODO
        panic!("TODO");
      }
      panic!("TODO");
  }

  pub fn put_mesh (&mut self, mesh: &Vec<Triangle3D>, cam: &Camera) {
    // TODO order, clipping, color
    for triangle in mesh {
      let transformed_triangle = triangle.clone()
      // let clipped_triangle_list = clip();
      
      // for clipped_triangle in clipped_triangle_list {
        // let transformed_triangle = clipped_triangle.clone()
          .translate(-1.0 * cam.position)
          .rotation_y(cam.yaw)
          .rotation_x(cam.pitch)
          .projection(cam.focal_length)
          .toScreen(self);
        self.put_triangle(&transformed_triangle, '@');
      // }
    }
  }
}


#[derive(Debug)]
pub struct Camera {
  pub position: Vec3,
  pub pitch: f32,
  pub yaw: f32,
  pub focal_length: f32
}

impl Camera {
  pub fn new (position: Vec3, pitch: f32, yaw: f32, focal_length: f32) -> Camera {
    if focal_length < 1.0 {
      panic!("The focal_length must be upper than 1.0");
    }
    Camera {position, pitch, yaw, focal_length}
  }
  pub fn get_look_at_direction () -> Vec3 {
    panic!("Not yet implemented")
  }
  pub fn get_forward_direction (&mut self) -> Vec3 {
    Vec3 {
      x: -f32::sin(self.yaw),
      y: 0.0,
      z: f32::cos(self.yaw)
    }
  }
  pub fn get_right_direction (&mut self) -> Vec3 {
    Vec3 {
      x: f32::cos(self.yaw), 
      y: 0.0,
      z: f32::sin(self.yaw)
    }
  }
  pub fn move_from_inputs (&mut self, delta_time: f32) -> io::Result<()> {
    let forward_direction = self.get_forward_direction();
    let right_direction = self.get_right_direction();
    match read()? {
      Event::Key(event) => {
        match event.code {
          KeyCode::Down => {
            if self.pitch >= 1.57 {
              self.pitch -= 0.01*delta_time;
            }
          },
          KeyCode::Up => {
            if self.pitch < 1.57 {
              self.pitch += 0.01*delta_time;
            }
          },
          KeyCode::Left => {
            self.yaw += 0.01*delta_time;
          },
          KeyCode::Right => {
            self.yaw -= 0.01*delta_time;
          },
          KeyCode::Char('z') => {
            self.position += forward_direction*0.01*delta_time;
          },
          KeyCode::Char('s') => {
            self.position -= forward_direction*0.01*delta_time;
          },
          KeyCode::Char('q') => {
            self.position -= right_direction*0.01*delta_time;
          },
          KeyCode::Char('d') => {
            self.position += right_direction*0.01*delta_time;
          },
          KeyCode::Char(' ') => {
            if (event.modifiers == KeyModifiers::CONTROL) {
              self.position.y -= 0.01*delta_time;
            } else {
              self.position.y += 0.01*delta_time;
            }
          },
          _ => {}
        }
      },
      _ => {},
    }
    Ok(())
  }
}
