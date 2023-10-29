#[derive(Debug)]
pub struct Configuration {
  pub width: usize,
  pub height: usize,
  pub buffer_size: usize
}

impl Configuration {
  pub fn new () -> Configuration {
    if let Some((w, h)) = term_size::dimensions() {
      Configuration {
        width: w,
        height: h - 1,
        buffer_size: w * (h - 1)
      }
    } else {
      panic!("Unable to get term size !")
    }
  }
  pub const fn empty () -> Configuration {
    Configuration { width: 0, height: 0, buffer_size: 0 }
  }
}