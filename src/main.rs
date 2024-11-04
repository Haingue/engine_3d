pub mod core;
pub mod tools;

use std::path::Path;
use std::str::FromStr;
use std::{env, io};

use crossterm::event::{read, Event, KeyCode};
use getopts::Options;
use tools::logger::Logger;

use crate::core::engine::Engine;

use crate::core::math::triangle::Triangle3D;
use crate::tools::configuration::Configuration;
use crate::tools::wavefront;


fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
}
fn main() {
  // Parse parameters
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let mut opts = Options::new();
  opts.optopt("o", "object_path", "set path to 3d object (wavefront format)", "obj/cube.obj");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!("{}", f.to_string()) }
  };

  println!("args: {:?}", args);
  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }
  let object_path_str: String = match matches.opt_str("o") {
    Some(path_str) => path_str,
    None => "obj/cube.obj".to_string(),
  };
  let mut object_path: &Path = Path::new(&object_path_str);
  
  // initialize 3d engine
  let configuration:Configuration = Configuration::new();
  let logger: Logger = Logger::new();
  let mut engine: Engine = Engine::new(configuration.width, configuration.height - 1, &logger);
  logger.log(format!("Welcome on Rust 3D engine !"));
  logger.log(format!("Configuration: {:?}", configuration));
  logger.log(format!("Engine: {:?}", engine.to_string()));

  // load object
  let object: Vec<Triangle3D> = wavefront::read_object_file(object_path, &logger)
    .expect("Error to read file");

  logger.log(format!("Triangle: {:?}", object));
  wait_key();
  engine.play_loop(object);
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
