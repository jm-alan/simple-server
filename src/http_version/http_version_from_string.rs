use super::HttpVersion;

impl From<&str> for HttpVersion {
  #[inline(always)]
  fn from(s: &str) -> Self {
    match s {
      "HTTP/1.0" => Self::Legacy,
      "HTTP/1.1" => Self::Modern,
      _ => Self::Unsupported,
    }
  }
}

impl From<String> for HttpVersion {
  #[inline(always)]
  fn from(s: String) -> Self {
    s.as_str().into()
  }
}

impl From<&String> for HttpVersion {
  #[inline(always)]
  fn from(s: &String) -> Self {
    s.as_str().into()
  }
}
