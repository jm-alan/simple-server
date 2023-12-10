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
