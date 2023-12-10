mod http_version_from_string;

#[derive(Debug, Clone, Copy)]
pub enum HttpVersion {
  Modern,
  Unsupported,
  Legacy,
}

impl ToString for HttpVersion {
  #[inline(always)]
  fn to_string(&self) -> String {
    match self {
      Self::Unsupported => "UNSUPPORTED".to_string(),
      Self::Legacy => "HTTP/1.0".to_string(),
      Self::Modern => "HTTP/1.1".to_string(),
    }
  }
}
