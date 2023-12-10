#[derive(Debug, Default, Clone, Copy)]
pub enum HttpMethod {
  #[default]
  GET,
  HEAD,
  POST,
  PUT,
  DELETE,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH,
}

impl ToString for HttpMethod {
  fn to_string(&self) -> String {
    match self {
      Self::GET => "GET".to_string(),
      Self::HEAD => "HEAD".to_string(),
      Self::POST => "POST".to_string(),
      Self::PUT => "PUT".to_string(),
      Self::DELETE => "DELETE".to_string(),
      Self::CONNECT => "CONNECT".to_string(),
      Self::OPTIONS => "OPTIONS".to_string(),
      Self::TRACE => "TRACE".to_string(),
      Self::PATCH => "PATCH".to_string(),
    }
  }
}

impl From<&str> for HttpMethod {
  fn from(value: &str) -> Self {
    match value {
      "HEAD" => Self::HEAD,
      "POST" => Self::POST,
      "PUT" => Self::PUT,
      "DELETE" => Self::DELETE,
      "CONNECT" => Self::CONNECT,
      "OPTIONS" => Self::OPTIONS,
      "TRACE" => Self::TRACE,
      "PATCH" => Self::PATCH,
      _ => Self::default(),
    }
  }
}

impl From<String> for HttpMethod {
  fn from(value: String) -> Self {
    value.as_str().into()
  }
}

impl From<&String> for HttpMethod {
  fn from(value: &String) -> Self {
    value.as_str().into()
  }
}
