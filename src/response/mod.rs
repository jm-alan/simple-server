mod http_body;
mod http_version;
mod response_to_string;
mod status_reason;

use self::{
  http_body::HttpBody, http_version::HttpVersion, status_reason::StatusReason,
};

#[derive(Debug, Default, Clone)]
pub struct Response {
  http_version: HttpVersion,
  status_reason: StatusReason,
  headers: Vec<String>,
  body: HttpBody,
}

impl Response {
  pub fn serialize(&self) -> Vec<u8> {
    self.to_string().as_bytes().to_owned()
  }
}
