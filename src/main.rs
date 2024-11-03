pub mod core;
pub mod tools;

use core::engine::LightSource;
use core::math::triangle;
use std::path::Path;
use std::time::Duration;
use std::{io, time};
use std::thread::sleep;

use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};

use crate::core::engine::Engine;
use std::fs::File;
use std::io::Write;
use std::ops::Mul;
use std::time::Instant;

use crate::core::engine::Camera;
use crate::core::math::triangle::Triangle3D;
use crate::core::math::vector::Vec3;
use crate::core::math::{vector::Vec2, triangle::Triangle2D};
use crate::tools::configuration::Configuration;
use crate::tools::wavefront;
fn main() {
    println!("Welcome on Rust 3D engine !");
    let configuration:Configuration = Configuration::new();
    println!("Configuration: {:?}", configuration);
    let mut engine: Engine = Engine::new(configuration.width, configuration.height - 1);
    println!("Engine: {:?}", engine.to_string());

    // demo(&mut engine, get_cude());

    // let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/cube.obj"))
    // let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/test.obj"))
    // let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/Home.obj"))
    let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/landscape.obj"))
        .expect("Error to read file");
    for triangle in object.clone() {
        println!("Triangle: {:?}", triangle);
    }
    wait_key();
    demo(&mut engine, object);
}

fn log (msg: String) -> Result<(), std::io::Error> {
    println!("{}", msg);
    // let mut f: File = File::options().append(true).open("engine_3D.log")?;
    // writeln!(&mut f, "{}", msg);
    Ok(())
}

fn  get_cude() -> Vec<Triangle3D> {
    vec![
        Triangle3D::new(
        Vec3::new(-0.5,    -0.5,   1.0),
        Vec3::new(-0.5,     0.5,   1.0),
        Vec3::new( 0.5,     0.5,   1.0)),
        Triangle3D::new(
        Vec3::new(-0.5,    -0.5,   1.0),
        Vec3::new( 0.5,     0.5,   1.0),
        Vec3::new( 0.5,    -0.5,   1.0)),
        Triangle3D::new(
        Vec3::new(-0.5,    -0.5,   4.0),
        Vec3::new(-0.5,     0.5,   4.0),
        Vec3::new( 0.5,     0.5,   4.0)),
        Triangle3D::new(
        Vec3::new(-0.5,    -0.5,   4.0),
        Vec3::new( 0.5,     0.5,   4.0),
        Vec3::new( 0.5,    -0.5,   4.0)),
        Triangle3D::new(
        Vec3::new(-0.5,    -0.5,   1.0),
        Vec3::new(0.5,     -0.5,   1.0),
        Vec3::new(0.5,     -0.5,   4.0)),
        Triangle3D::new(
        Vec3::new(-0.5,    -0.5,   1.0),
        Vec3::new(-0.5,    -0.5,   4.0),
        Vec3::new(0.5,     -0.5,   4.0)),
        Triangle3D::new(
        Vec3::new(-0.5,     0.5,   1.0),
        Vec3::new(0.5,      0.5,   1.0),
        Vec3::new(0.5,      0.5,   4.0)),
        Triangle3D::new(
        Vec3::new(-0.5,     0.5,   1.0),
        Vec3::new(-0.5,     0.5,   4.0),
        Vec3::new(0.5,      0.5,   4.0)),
    ]
}

fn wait_key () -> io::Result<()> {
    println!("\n\nPress the key 's' to start");
    /*
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    */
    loop {
        match read()? {
            Event::FocusGained => continue,
            Event::FocusLost => continue,
            Event::Key(event) => {
                if event.code == KeyCode::Char('s') {
                    break;
                }
                continue;
            },
            Event::Mouse(event) => continue,
            Event::Paste(data) => continue,
            Event::Resize(width, height) => continue,
        }
    }
    Ok(())
}

fn demo (engine: &mut Engine, object: Vec<Triangle3D>) -> io::Result<()> {
    let mut cam = Camera { position: Vec3 {x: 0.0, y:0.0, z: 0.0 }, pitch: 0.0, yaw: 0.0, focal_length: 1.0 };
    let mut last: Instant = Instant::now();
    let light_source: LightSource = LightSource::at(&Vec3::new(5.0, 5.0, 5.0));
    loop {
        let current_time: Instant = Instant::now();
        let delta_time: f32 = (current_time - last).as_millis() as f32;
        last = current_time;

        engine.clear(' ');
        if poll(Duration::from_millis(10))? {
            cam.move_from_inputs(delta_time);
        }
        engine.put_mesh(object.clone(), &cam, &light_source);
        engine.draw();
        log(format!("yaw: {:?}, pitch: {:?}, position: {:?}, delta_time= {:?}, current_time={:?}", cam.yaw, cam.pitch, cam.position, delta_time, (current_time - last).as_millis() as f32));
    }
}


