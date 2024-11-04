use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path, usize,
};

use crate::core::math::{
    triangle::Triangle3D,
    vector::Vec3,
};

use super::logger::Logger;

pub fn read_object_file(path: &Path, logger: &Logger) -> Result<Vec<Triangle3D>> {
    let mut triangles: Vec<Triangle3D> = vec![];

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Error to load line");
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        match tokens.get(0) {
            Some(&"v") => {
                logger.log(format!("vextex: {}", line));
                if let [_, x, y, z] = &tokens[..] {
                    let vertex = Vec3 {
                        x: x.parse()
                            .expect(&format!("Erreur de parsing pour le vertex '{}'", line)),
                        y: y.parse()
                            .expect(&format!("Erreur de parsing pour le vertex '{}'", line)),
                        z: z.parse()
                            .expect(&format!("Erreur de parsing pour le vertex '{}'", line)),
                    };
                    vertices.push(vertex);
                }
            }
            Some(&"f") => {
                logger.log(format!("face: {}", line));
                let face: std::result::Result<Vec<usize>, String> = tokens[1..]
                    .iter()
                    .fold(Ok(vec![]), |acc, &index| {
                      let parsed_index = index.parse::<usize>().map(|i| i - 1)
                        .map_err(|_| format!("Erreur de parsing pour l'indice '{}'", index));
                      match acc {
                          Ok(mut vec) => {
                              match parsed_index {
                                  Ok(val) => {
                                      vec.push(val);
                                      Ok(vec)
                                  },
                                  Err(e) => Err(e),
                              }
                          },
                          Err(e) => Err(e), // Si l'accumulateur est déjà une erreur, on la propage
                      }
                    });

                match face {
                    Ok(face_indices) => faces.push(face_indices),
                    Err(e) => (),
                }
            }
            _ => {}
        }
    }

    let mut triangles = Vec::new();
    for face in faces {
        match face.len() {
            3 => {
                triangles.push(Triangle3D {
                    v1: vertices[face[0]].clone(),
                    v2: vertices[face[1]].clone(),
                    v3: vertices[face[2]].clone(),
                });
            }
            4 => {
                triangles.push(Triangle3D {
                    v1: vertices[face[0]].clone(),
                    v2: vertices[face[1]].clone(),
                    v3: vertices[face[2]].clone(),
                });
                triangles.push(Triangle3D {
                    v1: vertices[face[2]].clone(),
                    v2: vertices[face[3]].clone(),
                    v3: vertices[face[0]].clone(),
                });
            }
            _ => {}
        }
    }

    Ok(triangles)
}
