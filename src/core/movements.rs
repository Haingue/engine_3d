use core::f32;

use super::{engine::{Camera}, math::vector::Vec3};


// cam.move_in_circle(center, radius, speed, delta_time, height);
pub fn cam_move_in_circle(mut cam: Camera, delta_time: f32) {
    let center = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let radius = 2.0;
    let speed = 0.0005; // radians par seconde
    let height = 0.0;

    // Calculer l'angle actuel en fonction du temps
    let angle = (speed * delta_time) % (2.0 * f32::consts::PI);

    // Calculer la nouvelle position de la caméra
    cam.position.x = center.x + radius * angle.cos();
    cam.position.z = center.z + radius * angle.sin();
    cam.position.y = center.y + height;

    // Garder la caméra focalisée sur le centre
    cam.yaw += angle*delta_time;
}