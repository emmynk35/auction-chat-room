#[derive(Debug)]
pub struct Modal {
  pub show: bool,
  pub location: String,
}

impl Default for Modal {
  fn default() -> Self {
    let modal = Self {
      show: false,
      location: "".to_string(),
    };
    modal
  }
}