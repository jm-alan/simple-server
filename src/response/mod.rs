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

impl Default for Response {
  fn default() -> Self {
    Self {
      http_version: HttpVersion::Modern,
      status_reason: StatusReason::Ok,
      headers: vec![],
      body: None,
    }
  }
}

impl Response {
  #[inline(always)]
  pub fn as_bytes(&self) -> Vec<u8> {
    self.to_string().into_bytes()
  }
}
