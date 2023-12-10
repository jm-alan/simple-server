use super::Response;

impl ToString for Response {
  #[inline(always)]
  fn to_string(&self) -> String {
    format!(
      "{} {}\r\n\r\n\r\n{}\r\n",
      self.http_version.to_string(),
      self.status_reason.to_string(),
      self.body.to_string(),
    )
  }
}
