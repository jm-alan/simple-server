use crate::http_version::HttpVersion;

use super::{status_reason::StatusReason, Response};

impl From<StatusReason> for Response {
  fn from(status_reason: StatusReason) -> Self {
    Self {
      http_version: HttpVersion::Modern,
      status_reason,
      ..Default::default()
    }
  }
}
