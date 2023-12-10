mod http_body;
mod response_to_string;
mod status_reason;

use super::http_version::HttpVersion;
use http_body::HttpBody;
use status_reason::StatusReason;

#[derive(Debug, Clone)]
pub struct Response {
  http_version: HttpVersion,
  status_reason: StatusReason,
  headers: Vec<(String, String)>,
  body: Option<HttpBody>,
}

impl Response {
  #[inline(always)]
  pub fn as_bytes(&self) -> &[u8] {
    self.to_string().as_bytes()
  }
}
