mod http_body;
mod response_to_string;
mod status_line;
mod status_reason;

use crate::{http_version::HttpVersion, types::Headers};

use http_body::HttpBody;
use status_reason::StatusReason;

#[derive(Debug, Clone)]
pub struct Response {
  http_version: HttpVersion,
  status_reason: StatusReason,
  headers: Headers,
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

  #[inline(always)]
  pub fn bad_request() -> Self {
    Self {
      http_version: HttpVersion::Modern,
      status_reason: StatusReason::BadRequest,
      ..Default::default()
    }
  }
}
