use std::{io::{self}, time::{Duration, Instant}};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};

use super::math::{math::cross_prod, triangle::{Triangle2D, Triangle3D}, vector::{Vec2, Vec3}};
use crate::{core::math::math::{dot, line_plane_intersection}, tools::logger::Logger};

#[derive(Debug)]
pub struct Engine<'a> {
  pub width: usize,
  pub height: usize,
  pub pixel_ratio: usize,
  pub pixel_buffer_size: usize,
  pub pixel_buffer: Vec<char>,
  pub logger: &'a Logger
}

impl ToString for Engine<'_> {
    fn to_string(&self) -> String {
        format!("Engine [width={}, height={}, pixel_buffer_size={}]", self.width, self.height, self.pixel_buffer_size)
    }
}

impl Engine<'_> {
  pub fn new (width: usize, height: usize, logger: &Logger) -> Engine {
    if width < 1 || height < 1 {
      panic!("The width or height must be upper than 0");
    }
    Engine {
      width: width.clone(),
      height: height.clone(),
      pixel_ratio: 29/13,
      pixel_buffer_size: width * height,
      pixel_buffer: vec![' ' ; width * height],
      logger
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
    fn in_z (normal_plane: Vec3, normal_point: Vec3, triangle: Triangle3D) -> (Vec<Vec3>, Vec<Vec3>, bool) {
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
      return (out, in_, vert1*vert3 > 0.0);
    }
    let z_near: Vec3 = cam.position + 0.1 * normal_plane;
    let (out, in_, is_inverted) = in_z(normal_plane, z_near, triangle);
    if out.len() == 0 {
      return vec![triangle];
    } else if out.len() == 3 {
        return vec![];
    } else if out.len() == 1 {
      let collision0 = line_plane_intersection(normal_plane, z_near, out[0], in_[0]);
      let collision1 = line_plane_intersection(normal_plane, z_near, out[0], in_[1]);
      if is_inverted {
        return vec![
          Triangle3D::new(collision1, in_[1], collision0),
          Triangle3D::new(collision0, in_[1], in_[0]),
        ];
      } else {
        return vec![
          Triangle3D::new(collision0, in_[0], collision1),
          Triangle3D::new(collision1, in_[0], in_[1]),
        ];
      }
    } else if out.len() == 2 {
      if is_inverted {
        return vec![
          Triangle3D::new(
            line_plane_intersection(normal_plane, z_near, out[0], in_[0]),
            in_[0],
            line_plane_intersection(normal_plane, z_near, out[1], in_[0]),
          )
        ];

      } else {
        return vec![
          Triangle3D::new(
            line_plane_intersection(normal_plane, z_near, out[0], in_[0]),
            line_plane_intersection(normal_plane, z_near, out[1], in_[0]),
            in_[0]
          )
        ];
      }
    }
    panic!("TODO");
  }

  pub fn distance_triangle_camera (&self, triangle: Triangle3D, cam: &Camera) -> f32 {
    let position: Vec3 = ((triangle.v1+triangle.v2+triangle.v3)*(1.0/3.0)) - cam.position;
    return position.length2();
  }

  pub fn put_mesh (&mut self, mut mesh: Vec<Triangle3D>, cam: &Camera, light_source: &LightSource) {
    // sort triangle by distance to draw near traiangle at the end
    mesh.sort_by(
      |&a, &b| {
        let distance_a = self.distance_triangle_camera(a, cam);
        let distance_b = self.distance_triangle_camera(b, cam);
        distance_b.partial_cmp(&distance_a).unwrap_or(std::cmp::Ordering::Equal)
      });
    let look_at: Vec3 = cam.get_look_at_direction();
    for triangle in mesh {
      // add "Clipping" avoid triangle bug due to the camera
      let clipped_triangle_list = self.clip(triangle, cam, look_at);
      
      for clipped_triangle in clipped_triangle_list {
        let line1 : Vec3 = clipped_triangle.v2 - clipped_triangle.v1;
        let line2 : Vec3 = clipped_triangle.v3 - clipped_triangle.v1;
        let surface_normal: Vec3 = cross_prod(line1, line2);
        
        // add "Face-Culling" to reduce the number of triangle drawn
        if true || dot(surface_normal, clipped_triangle.v1 - cam.position) < 0.0 {
          // add light based on the light source and the triangle position
          let light_char: char = light_source.diffuse_light(surface_normal, clipped_triangle.v1);
          let transformed_triangle = clipped_triangle.clone()
            .translate(-1.0 * cam.position)
            .rotation_y(cam.yaw)
            .rotation_x(cam.pitch)
            .projection(cam.focal_length)
            .to_screen(self);
          self.put_triangle(&transformed_triangle, light_char);
        }
      }
    }
  }

  pub fn play_loop (&mut self, object: Vec<Triangle3D>) -> io::Result<()> {
    let mut cam = Camera { position: Vec3 {x: 1.0, y: 0.0, z: -1.0 }, pitch: 0.1, yaw: 0.5, focal_length: 1.0 };
    let mut last: Instant = Instant::now();
    let light_source: LightSource = LightSource::at(&Vec3::new(5.0, 5.0, 5.0));
    loop {
      let current_time: Instant = Instant::now();
      let delta_time: f32 = (current_time - last).as_millis() as f32;
      last = current_time;

      self.clear(' ');
      if poll(Duration::from_millis(10))? {
        match cam.move_from_inputs(delta_time) {
          Ok(()) => (),
          Err(error) => {
            self.logger.log(format!("Input error: {:?}", error));
          }
        }
      }
      self.put_mesh(object.clone(), &cam, &light_source);
      self.draw();
      self.logger.log(format!("yaw: {:?}, pitch: {:?}, position: {:?}, delta_time= {:?}, current_time={:?}", cam.yaw, cam.pitch, cam.position, delta_time, (current_time - last).as_millis() as f32));
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
  pub fn get_look_at_direction (&self) -> Vec3 {
    Vec3 {
      x: (-f32::sin(self.yaw)*f32::cos(self.pitch)),
      y: (f32::sin(self.pitch)),
      z: (f32::cos(self.yaw)*f32::cos(self.pitch)) }
  }
  pub fn get_forward_direction (&self) -> Vec3 {
    Vec3 {
      x: -f32::sin(self.yaw),
      y: 0.0,
      z: f32::cos(self.yaw)
    }
  }
  pub fn get_right_direction (&self) -> Vec3 {
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
            if event.modifiers == KeyModifiers::CONTROL {
              self.position.y -= 0.01*delta_time;
            } else {
              self.position.y += 0.01*delta_time;
            }
          },
          _ => {}
        }
      },
      // Event::Mouse(event) => {},
      // Event::Paste(data) => {},
      // Event::Resize(width, height) => {},
      // Event::FocusGained => {},
      // Event::FocusLost => {},
      _ => {},
    }
    Ok(())
  }
}

pub struct LightSource {
  light_gradient: Vec<char>,
  pub position: Vec3,
}

impl LightSource {
  pub fn new () -> LightSource {
    LightSource {
      light_gradient: vec!['.', ',', ';', 'l', 'a', '#', '@'],
      position: Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
  }
  pub fn at (position: &Vec3) -> LightSource {
    LightSource {
      light_gradient: vec!['.', ',', ';', 'l', 'a', '#', '@'],
      position: *position
    }
  }
  pub fn diffuse_light (&self, normal_surface: Vec3, vertex: Vec3) -> char {
    let light_direction: Vec3 = self.position - vertex;
    let intensity: f32 = dot(light_direction.normalize(), normal_surface.normalize());
    if intensity >= 0.0 {
      let char_number: f32 = self.light_gradient.len() as f32 - 1.0;
      let symbol_idx_real: f32 = intensity * (char_number);
      let symbol_idx: usize = symbol_idx_real.round() as usize;
      return self.light_gradient[symbol_idx]
    }
    return '.'
  }
}