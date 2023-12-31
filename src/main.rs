pub mod core;
pub mod tools;

use std::{io, time};
use std::thread::sleep;

use crate::core::engine::Engine;
use crate::core::math::{Vec2, Triangle};
use crate::tools::configuration::{Configuration};

fn main() {
    println!("Welcome on Rust 3D engine !");
    let configuration:Configuration = Configuration::new();
    println!("Configuration: {:?}", configuration);
    let mut engine: Engine = Engine::new(configuration.width, configuration.height - 1);
    println!("Engine: {:?}", engine.to_string());

    wait_key();

    let mut triangle_1 = Triangle::new(Vec2::new(1, 3), Vec2::new(1, 8), Vec2::new(20, 20));
    let mut triangle_2 = Triangle::new(Vec2::new(100, 1), Vec2::new(80, 5), Vec2::new(90, 10));
    println!("Try triangle: {:?}", triangle_1);
    for _frame in 1..50 {
        engine.clear(' ');
        triangle_1.update_p2(triangle_1.p2.x+1, triangle_1.p2.y);
        triangle_2.update_p2(triangle_2.p2.x.saturating_sub(1), triangle_2.p2.y);
        engine.put_triangle(&triangle_1, 'O');
        engine.put_triangle(&triangle_2, '@');
        engine.draw();
        sleep(time::Duration::from_millis(10))
    }
}

fn wait_key () {
    println!("\n\nPress a key to start");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
}