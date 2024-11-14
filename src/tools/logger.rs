use std::io::Write;
use std::path::Path;
use std::fs::File;


#[derive(Default, Debug)]
pub struct Logger {
  pub log_file_path: String,
  pub log_file: Option<File>,
  pub enable: bool
}


impl Logger {
  pub fn new () -> Logger {
    let log_file_path: &str = "engine_3D.log";
    let log_file: File = File::options().append(true).open(Path::new(log_file_path))
      .expect("Error to open log file");

    Logger {
      log_file_path: String::from(log_file_path),
      log_file: Some(log_file),
      enable: false
    }
  }

  pub fn from (log_file_path: &Path) -> Logger {
    let log_file: File = File::options().append(true).open(log_file_path)
      .expect("Error to open log file");

    Logger {
      log_file_path: String::from(log_file_path.to_str().unwrap()),
      log_file: Some(log_file),
      enable: false
    }
  }

  pub fn enable_log (&mut self) {
    self.enable = true
  }

  pub fn disable_log (&mut self) {
    self.enable = false
  }

  pub fn log (&self, msg: String) {
    if self.enable {
      if self.log_file.is_some() {
        let mut file: &File = self.log_file
          .as_ref()
          .expect("Error to retrieve the file");
        match writeln!(file, "{}", msg) {
          _ => ()
        }
      }
    }
  }
}

impl std::string::ToString for Logger {
    fn to_string(&self) -> String {
        format!("Logger: [path={}]", self.log_file_path)
    }
}