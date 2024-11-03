use std::{fs::File, io::{self, BufRead, Result}, path::Path};

use crate::core::math::{triangle::{self, Triangle3D}, vector::Vec3};

pub fn read_object_file (path: &Path) -> Result<Vec<Triangle3D>> {
  let file = File::open(path)?;
  let reader = io::BufReader::new(file);

  let mut vertices: Vec<Vec3> = vec![];
  let mut triangles: Vec<Triangle3D> = vec![];
  for line_result in reader.lines() {
    if line_result.is_err() {continue}
    
    let line = line_result?;
    if line.starts_with("v") {
      // vertex
      println!("vextex: {}", line);
      let mut token_iterator = line.split_whitespace();
      token_iterator.next();
      let x_str = token_iterator.next().expect("Missing token for x");
      let y_str = token_iterator.next().expect("Missing token for y");
      let z_str = token_iterator.next().expect("Missing token for z");
      let x = x_str.parse().expect("Error to parse the x value");
      let y = y_str.parse().expect("Error to parse the y value");
      let z = z_str.parse().expect("Error to parse the z value");
      vertices.push(Vec3::new(x, y, z));
    } else if line.starts_with("f") {
      // face
      println!("face: {}", line);
      let mut token_iterator = line.split_whitespace();
      token_iterator.next();
      let mut vertex_number: usize = 0;
      let mut triangle_vertex: Vec<usize> = vec![];
      
      let mut v1: Vec3;
      let mut v2: Vec3;
      let mut v3: Vec3;
      let mut first_vertex: usize = usize::MAX;
      for vertex in token_iterator {
        vertex_number += 1;
        let current_vertex_idx: usize = vertex.parse().expect("Error to parse vertex number");
        triangle_vertex.push(current_vertex_idx);
        if (first_vertex == usize::MAX) {
          first_vertex = current_vertex_idx;
        }
        if vertex_number % 3 == 0 {
          // create triangle
          v1 = vertices[triangle_vertex[0]-1];
          v2 = vertices[triangle_vertex[1]-1];
          v3 = vertices[triangle_vertex[2]-1];
          triangles.push(Triangle3D::new(v1, v2, v3));

          let last_vertex: usize = *triangle_vertex.last().expect("No vertex");
          triangle_vertex = vec![last_vertex];
          vertex_number += 1;
        }
      }
      if triangle_vertex.len() % 3 != 0 {
        // get last vertex
        v1 = vertices[triangle_vertex[0]-1];
        v2 = vertices[triangle_vertex[1]-1];
        v3 = vertices[first_vertex-1]; // first vertex
        triangles.push(Triangle3D::new(v1, v2, v3));
      }
    }
  }

  Ok(triangles)
}