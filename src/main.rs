pub mod core;
pub mod tools;

use core::engine::LightSource;
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

fn main() {
    println!("Welcome on Rust 3D engine !");
    let configuration:Configuration = Configuration::new();
    println!("Configuration: {:?}", configuration);
    let mut engine: Engine = Engine::new(configuration.width, configuration.height - 1);
    println!("Engine: {:?}", engine.to_string());

    // loop_print_events();
    // wait_key();

    // demo_1(&mut engine);
    // demo_2(&mut engine);
    // demo_3(&mut engine);
    // demo_4(&mut engine);
    // demo_5(&mut engine);
    // demo_6(&mut engine);
    demo_7(&mut engine);
    
}

fn demo_1 (engine: &mut Engine) {
    wait_key();
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

    // loop_print_events();
    return
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
/*
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
        Vec3::new( -0.5,    -0.5,   0.0),
        Vec3::new(  0.0,    0.5,    0.0),
        Vec3::new(  0.5,    -0.5,   0.0));
    let mut triangle_2 = Triangle3D::new(
        Vec3::new( -0.5,    -0.5,   0.0),
        Vec3::new(  0.0,    0.5,    0.0),
        Vec3::new(  0.5,    -0.5,   0.0));
    let mut triangle_3 = Triangle3D::new(
        Vec3::new( -0.5,    -0.5,   0.0),
        Vec3::new(  0.0,    0.5,    0.0),
        Vec3::new(  0.5,    -0.5,   0.0));
    println!("Try triangle: {:?}", triangle_1);
    let mut last = Instant::now();
    let mut rotation: f32 = 0.0;
    let mut distance: f32 = 1.0;
    let mut direction: bool = false;
    loop {
        let current = Instant::now();
        let dt = current.duration_since(last).as_secs_f32().mul(100.0);
        if direction {
            distance-=0.001;
            if distance < 0.5 {
                direction = false
            }
        } else {
            distance+=0.001;
            if distance > 1.5 {
                direction = true
            }
        }
        let _ = log(format!("last: {:?}, current: {:?}, dt: {:?}, rot: {:?}, nrot: {:?}, time: {:?}", last, current, dt, rotation, 0.001 * dt, Instant::now().elapsed().as_nanos()));
        last = current;
        rotation += 0.01 * dt;
        // rotation += 0.001;
        triangle_1.v1.z=distance;
        triangle_1.v2.z=distance;
        triangle_1.v3.z=distance;
        engine.clear(' ');
        engine.put_triangle(&triangle_1
            .rotation_y(rotation)
            .rotation_x(-rotation)
            .translate(Vec3{x:4.0, y:0.0, z:2.0})
            .projection(1.0)
            .toScreen(&engine), '@');
        engine.put_triangle(&triangle_2
            .rotation_y(rotation)
            .translate(Vec3{x:-4.0, y:0.0, z:2.0})
            .projection(1.0)
            .toScreen(&engine), '@');
        engine.put_triangle(&triangle_3
            .rotation_x(rotation)
            .translate(Vec3{x:0.0, y:0.0, z:2.0})
            .projection(1.0)
            .toScreen(&engine), '@');
        engine.draw();
        sleep(time::Duration::from_millis(1))
    }
}

fn demo_5 (engine: &mut Engine) {
    let mut carreMesh = vec![
        Triangle3D::new(
            Vec3::new( -0.5,    -0.5,   1.0),
            Vec3::new(  -0.5,    0.5,    1.0),
            Vec3::new(  0.5,    0.5,   1.0)),
        Triangle3D::new(
            Vec3::new( -0.5,    -0.5,   1.0),
            Vec3::new(  0.5,    0.5,    1.0),
            Vec3::new(  0.5,    -0.5,   1.0))
    ];
    let mut cam = Camera { position: Vec3 {x: 0.0, y:0.0, z: 0.0 }, pitch: 0.0, yaw: 0.0, focal_length: 1.0 };
    let mut last = Instant::now();
    loop {
        for _ in 0..300 {
            log(format!("yaw: {:?}, pitch: {:?}, position: {:?}", cam.yaw, cam.pitch, cam.position));
            engine.clear(' ');
            let current = Instant::now();
            let dt = current.duration_since(last).as_secs_f32().mul(100.0);
            cam.yaw+=0.00001 * dt;
            engine.put_mesh(&carreMesh, &cam);
            engine.draw();
            println!("pos: {}", cam.position.z);
        }
        for _ in 0..300 {
            log(format!("yaw: {:?}, pitch: {:?}, position: {:?}", cam.yaw, cam.pitch, cam.position));
            engine.clear(' ');
            let current = Instant::now();
            let dt = current.duration_since(last).as_secs_f32().mul(100.0);
            cam.yaw-=0.00001 * dt;
            engine.put_mesh(&carreMesh, &cam);
            engine.draw();
            println!("pos: {}", cam.position.z);
        }
    }
}

fn demo_6 (engine: &mut Engine) {
    let mut mesh = vec![];
    mesh.push(Triangle3D::new(
        Vec3::new( -0.5,    -0.5,   0.50),
        Vec3::new(  0.0,    0.5,    0.50),
        Vec3::new(  0.5,    -0.5,   0.50)));
    let mut cam = Camera { position: Vec3 {x: 0.0, y:0.0, z: 0.0 }, pitch: 0.0, yaw: 0.0, focal_length: 1.0 };
    loop {
        engine.clear(' ');
        cam_inputs(&mut cam);
        engine.put_mesh(&mesh, &cam);
        engine.draw();
    }
}
*/
fn demo_7 (engine: &mut Engine) -> io::Result<()> {
    let mut carre = vec![
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
    ];
    let mut cam = Camera { position: Vec3 {x: 0.0, y:0.0, z: 0.0 }, pitch: 0.0, yaw: 0.0, focal_length: 1.0 };
    let mut last: Instant = Instant::now();
    let light_source: LightSource = LightSource::new();
    loop {
        let current_time: Instant = Instant::now();
        let delta_time: f32 = (current_time - last).as_millis() as f32;
        last = current_time;

        engine.clear(' ');
        if poll(Duration::from_millis(10))? {
            cam.move_from_inputs(delta_time);
        }
        engine.put_mesh(&carre, &cam, &light_source);
        engine.draw();
        log(format!("yaw: {:?}, pitch: {:?}, position: {:?}, delta_time= {:?}, current_time={:?}", cam.yaw, cam.pitch, cam.position, delta_time, (current_time - last).as_millis() as f32));
    }
}

fn cam_inputs (cam: &mut Camera) -> io::Result<()> {
    match read()? {
        Event::Key(event) => {
            match event.code {
                KeyCode::Down => {
                    if cam.pitch >= 1.57 {
                        cam.pitch -= 0.01;
                    }
                },
                KeyCode::Up => {
                    if cam.pitch < 1.57 {
                        cam.pitch += 0.01;
                    }
                },
                KeyCode::Left => {
                    cam.yaw += 0.01;
                },
                KeyCode::Right => {
                    cam.yaw -= 0.01;
                },
                KeyCode::Char('z') => {
                    cam.position.z += 0.1;
                },
                KeyCode::Char('s') => {
                    cam.position.z -= 0.1;
                },
                KeyCode::Char('d') => {
                    cam.position.x -= 0.1;
                },
                KeyCode::Char('q') => {
                    cam.position.x += 0.1;
                },
                KeyCode::Char(' ') => {
                    if (event.modifiers == KeyModifiers::CONTROL) {
                        cam.position.y -= 0.1;
                    } else {
                        cam.position.y += 0.1;
                    }
                },
                _ => {}
            }
            log(format!("KeyEvent: [code={:?}, modifier={:?}]", event.code, event.modifiers));
        },
        _ => {},
    }
    Ok(())
}

fn rotation_inputs (vec: &mut Vec3) -> io::Result<()> {
    match read()? {
        Event::Key(event) => {
            match event.code {
                KeyCode::Down => {
                    vec.rotation_x(vec.x - 0.01);
                },
                KeyCode::Up => {
                    vec.rotation_x(vec.x + 0.01);
                },
                KeyCode::Left => {
                    vec.rotation_y(vec.y + 0.01);
                },
                KeyCode::Right => {
                    vec.rotation_y(vec.y - 0.01);
                }
                _ => {}
            }
            log(format!("KeyEvent: [code={:?}, modifier={:?}]", event.code, event.modifiers));
        },
        _ => {},
    }
    Ok(())
}

fn log (msg: String) -> Result<(), std::io::Error> {
    println!("{}", msg);
    // let mut f: File = File::options().append(true).open("engine_3D.log")?;
    // writeln!(&mut f, "{}", msg);
    Ok(())
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

fn loop_print_events() -> io::Result<()> {
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(5000))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::FocusGained => {
                    println!("FocusGained");
                    break;
                },
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                Event::Paste(data) => println!("Pasted {:?}", data),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
    Ok(())
}