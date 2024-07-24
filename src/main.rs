pub mod core;
pub mod tools;

use std::time::Duration;
use std::{default, io, time};
use std::thread::sleep;

use crossterm::event::{poll, read, Event, KeyCode};

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

    // loop_print_events();
    return
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