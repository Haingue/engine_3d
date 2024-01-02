pub mod core;
pub mod tools;

use std::sync::mpsc::Receiver;
use std::{io, time, pin};
use std::thread::sleep;

use crate::core::engine::{Engine, Camera};
use crate::core::math::triangle::Triangle3D;
use crate::core::math::vector::Vec3;
use crate::core::math::{vector::Vec2, triangle::Triangle2D};
use crate::tools::configuration::{Configuration};

fn main() {
    println!("Welcome on Rust 3D engine !");
    let configuration:Configuration = Configuration::new();
    println!("Configuration: {:?}", configuration);
    let mut engine: Engine = Engine::new(configuration.width, configuration.height - 1);
    println!("Engine: {:?}", engine.to_string());
   
    // demo_1(&mut engine);
    // demo_2(&mut engine);
    // demo_3(&mut engine);
    // demo_4(&mut engine);
    demo_5(&mut engine);
}

fn demo_1 (engine: &mut Engine) {
    engine.wait_key();
    let mut triangle_1 = Triangle2D::new(Vec2::new(1.0, 3.0), Vec2::new(1.0, 8.0), Vec2::new(20.0, 20.0));
    let mut triangle_2 = Triangle2D::new(Vec2::new(100.0, 1.0), Vec2::new(80.0, 5.0), Vec2::new(90.0, 10.0));
    println!("Try triangle: {:?}", triangle_1);
    for _frame in 1..50 {
        engine.clear(' ');
        triangle_1.update_v2(triangle_1.v2.x+1.0, triangle_1.v2.y);
        triangle_2.update_v2(triangle_2.v2.x.ceil(), triangle_2.v2.y);
        engine.put_triangle(&triangle_1, 'O');
        engine.put_triangle(&triangle_2, '@');
        engine.draw();
        sleep(time::Duration::from_millis(10))
    }
}

fn demo_2 (engine: &mut Engine) {
    let mut triangle_1 = Triangle2D::new(
        Vec2::new( -0.5, -0.5),
        Vec2::new(0.0, 0.5),
        Vec2::new(0.5, -0.5));
    println!("Try triangle: {:?}", triangle_1);
    loop {
        engine.clear(' ');
        engine.put_triangle(&triangle_1.toScreen(&engine), '@');
        engine.draw();
        sleep(time::Duration::from_millis(10))
    }
}

fn demo_3 (engine: &mut Engine) {
    let mut triangle_1 = Triangle3D::new(
        Vec3::new( -0.5,    -0.5,   1.0),
        Vec3::new(  0.0,    0.5,    1.0),
        Vec3::new(  0.5,    -0.5,   1.0));
    println!("Try triangle: {:?}", triangle_1);
    for _frame in 1..10 {
        engine.clear(' ');
        engine.put_triangle(&triangle_1.projection(1.0).toScreen(&engine), '@');
        engine.draw();
        triangle_1.translate(Vec3{x:0.5, y:0.5, z:0.5});
        sleep(time::Duration::from_millis(100))
    }
}

fn demo_4 (engine: &mut Engine) {
    let mut triangle_1 = Triangle3D::new(
        Vec3::new( -0.5,    -0.5,   0.50),
        Vec3::new(  0.0,    0.5,    0.50),
        Vec3::new(  0.5,    -0.5,   0.50));
    println!("Try triangle: {:?}", triangle_1);
    let mut t: f32 = 0.0;
    loop {
        engine.clear(' ');
        t += 0.01;
        engine.put_triangle(&triangle_1
            .rotation_y(t)
            .translate(Vec3{x:0.0, y:0.0, z:2.0})
            .projection(1.0)
            .toScreen(&engine), '@');
        engine.draw();
        //sleep(time::Duration::from_millis(100))
    }
}

fn demo_5 (engine: &mut Engine) {
    let mut mesh = vec![];
    let mut triangle_1 = Triangle3D::new(
        Vec3::new( -0.5,    -0.5,   0.50),
        Vec3::new(  0.0,    0.5,    0.50),
        Vec3::new(  0.5,    -0.5,   0.50));
    mesh.push(triangle_1);
    println!("Try triangle: {:?}", triangle_1);
    let cam = Camera { position: Vec3 {x: 0.0, y:0.0, z: 0.0 }, pitch: 0.0, yaw: 0.0, focal_length: 1.0 };
    loop {
        engine.clear(' ');
        engine.put_mesh(&mesh, &cam);
        engine.draw();
        //sleep(time::Duration::from_millis(100))
    }
}

fn inputs (cam: &Camera) {
    // fn spawn_stdin_channel() -> Receiver<String> {
    //     let (tx, rx) = mpsc::channel::<String>();
    //     thread::spawn(move || loop {
    //         let mut buffer = String::new();
    //         io::stdin().read_line(&mut buffer).unwrap();
    //         tx.send(buffer).unwrap();
    //     });
    //     rx
    // }

    // let stdin_channel = spawn_stdin_channel();
    // loop {
    //     match stdin_channel.try_recv() {
    //         Ok(key) => {
    //             if key === "down" {
    //                 if cam.pitch >= 1.57 {
    //                     cam.pitch -= 0.01;
    //                 }
    //             }
    //             if key === "up" {
    //                 if cam.pitch < 1.57 {
    //                     cam.pitch += 0.01;
    //                 }
    //             }
    //             if key === "left" {
    //                 cam.yaw += 0.01;
    //             }
    //             if key === "right" {
    //                 cam.yaw -= 0.01;
    //             }
    //         },
    //         Err(TryRecvError::Empty) => println!("Channel empty"),
    //         Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
    //     }
    //     sleep(1000);
    // }
}