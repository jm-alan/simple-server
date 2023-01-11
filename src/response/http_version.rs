#[derive(Debug, Default, Clone)]
pub enum HttpVersion {
  Unsupported,
  Legacy,
  #[default]
  Modern,
}

impl ToString for HttpVersion {
  fn to_string(&self) -> String {
    match self {
      Self::Unsupported => "UNSUPPORTED".to_string(),
      Self::Legacy => "HTTP/1.0".to_string(),
      Self::Modern => "HTTP/1.1".to_string(),
    }
  }
}

impl From<&str> for HttpVersion {
  fn from(s: &str) -> Self {
    match s {
      "HTTP/1.0" => Self::Legacy,
      "HTTP/1.1" => Self::Modern,
      _ => Self::Unsupported,
    }
  }
}

impl From<String> for HttpVersion {
  fn from(s: String) -> Self {
    s.as_str().into()
  }
}

impl From<&String> for HttpVersion {
  fn from(s: &String) -> Self {
    s.as_str().into()
  }
}
