use std::io;

use crossterm::event::{read, Event, KeyCode, KeyModifiers};

use super::{engine::{Camera, LightSource}, math::triangle::Triangle3D};


pub fn player_action (camera: &mut Camera, light_source: &mut LightSource, objects: &mut Vec<Triangle3D>, delta_time: f32) -> io::Result<()> {
    let forward_direction = camera.get_forward_direction();
    let right_direction = camera.get_right_direction();
    match read()? {
      Event::Key(event) => {
        match event.code {
          KeyCode::Down => {
            if camera.pitch > -1.57 {
              camera.pitch -= 0.01*delta_time;
            }
          },
          KeyCode::Up => {
            if camera.pitch < 1.57 {
              camera.pitch += 0.01*delta_time;
            }
          },
          KeyCode::Left => {
            camera.yaw += 0.01*delta_time;
          },
          KeyCode::Right => {
            camera.yaw -= 0.01*delta_time;
          },
          KeyCode::Char('z') => {
            camera.position += forward_direction*0.01*delta_time;
          },
          KeyCode::Char('s') => {
            camera.position -= forward_direction*0.01*delta_time;
          },
          KeyCode::Char('q') => {
            camera.position -= right_direction*0.01*delta_time;
          },
          KeyCode::Char('d') => {
            camera.position += right_direction*0.01*delta_time;
          },
          KeyCode::Char(' ') => {
            if event.modifiers == KeyModifiers::CONTROL {
              camera.position.y -= 0.01*delta_time;
            } else {
              camera.position.y += 0.01*delta_time;
            }
          },
          KeyCode::Char('r') => {
            objects.iter_mut().for_each(|o| *o = o.rotation_y(0.2));
          },
          KeyCode::Char('R') => {
            objects.iter_mut().for_each(|o| *o = o.rotation_y(-0.2));
          },
          KeyCode::Char('t') => {
            light_source.move_in_circle(delta_time);
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