#[derive(Debug, Default, Clone)]
pub enum HttpBody {
  #[default]
  None,
  Html(String),
  Json(String),
}

impl ToString for HttpBody {
  fn to_string(&self) -> String {
    match self {
      Self::None => "".to_string(),
    }
  }
}
