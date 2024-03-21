use super::status_reason::StatusReason;
use crate::http_version::HttpVersion;

pub struct StatusLine {
  http_version: HttpVersion,
  status_reason: StatusReason,
}
