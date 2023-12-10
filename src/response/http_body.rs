use crate::content_type::ContentType;

#[derive(Debug, Default, Clone)]
pub struct HttpBody {
  content_type: ContentType,
  raw_data: String,
}

impl ToString for HttpBody {
  #[inline(always)]
  fn to_string(&self) -> String {
    format!(
      "Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
      self.content_type.to_string(),
      self.raw_data.len(),
      self.raw_data
    )
  }
}
