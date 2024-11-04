pub mod core;
pub mod tools;

use core::engine::LightSource;
use std::path::Path;
use std::time::Duration;
use std::io;

use crossterm::event::{poll, read, Event, KeyCode};
use tools::logger::Logger;

use crate::core::engine::Engine;
use std::time::Instant;

use crate::core::engine::Camera;
use crate::core::math::triangle::Triangle3D;
use crate::core::math::vector::Vec3;
use crate::tools::configuration::Configuration;
use crate::tools::wavefront;

fn main() {
    let configuration:Configuration = Configuration::new();
    let logger: Logger = Logger::new();
    let mut engine: Engine = Engine::new(configuration.width, configuration.height - 1);
    logger.log(format!("Welcome on Rust 3D engine !"));
    logger.log(format!("Configuration: {:?}", configuration));
    logger.log(format!("Engine: {:?}", engine.to_string()));

    // let object: Vec<Triangle3D> = get_cude();
    // let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/cube.obj"), &logger)
    // let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/test.obj"), &logger)
    // let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/Home.obj"), &logger)
    let object: Vec<Triangle3D> = wavefront::read_object_file(Path::new("obj/landscape.obj"), &logger)
        .expect("Error to read file");

    logger.log(format!("Triangle: {:?}", object));
    wait_key();
    play_loop(&mut engine, object, &logger);
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
    println!("\n\nPress the key 'enter' to start");
    loop {
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Enter {
                    break;
                }
                continue
            },
            _ => continue
        }
    }
    Ok(())
}

fn play_loop (engine: &mut Engine, object: Vec<Triangle3D>, logger: &Logger) -> io::Result<()> {
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
        logger.log(format!("yaw: {:?}, pitch: {:?}, position: {:?}, delta_time= {:?}, current_time={:?}", cam.yaw, cam.pitch, cam.position, delta_time, (current_time - last).as_millis() as f32));
    }
}
