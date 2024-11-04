use std::io::Write;
use std::path::Path;
use std::fs::File;


#[derive(Debug)]
pub struct Logger {
  pub log_file: Option<File>
}


impl Logger {
  pub fn new () -> Logger {
    let log_file: File = File::options().append(true).open(Path::new("engine_3D.log"))
      .expect("Error to open log file");

    Logger {
      log_file: Some(log_file)
    }
  }

  pub fn from (log_file_path: &Path) -> Logger {
    let log_file: File = File::options().append(true).open(log_file_path)
      .expect("Error to open log file");

    Logger {
      log_file: Some(log_file)
    }
  }

  pub fn log (&self, msg: String) -> Result<(), std::io::Error> {
      if self.log_file.is_some() {
        let mut file: &File = self.log_file
          .as_ref()
          .expect("Error to retrieve the file");
        writeln!(file, "{}", msg);
      }
      Ok(())
  }
}

