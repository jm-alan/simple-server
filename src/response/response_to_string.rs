use super::Response;

impl ToString for Response {
  #[inline(always)]
  fn to_string(&self) -> String {
    let formatted_headers = if self.headers.len() > 0 {
      self
        .headers
        .iter()
        .map(|(key, val)| format!("{key}: {val}\r\n"))
        .collect::<Vec<_>>()
        .join("")
    } else {
      "".into()
    };

    let formatted_body = match self.body {
      Some(ref body) => body.to_string(),
      _ => "\r\n".to_string(),
    };

    format!(
      "{} {}\r\n{}{}",
      self.http_version.to_string(),
      self.status_reason.to_string(),
      formatted_headers,
      formatted_body,
    )
  }
}
