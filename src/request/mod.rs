use crate::http_version::HttpVersion;

mod request_from_tcp_stream;

#[derive(Debug, Clone)]
pub struct Request {
  raw_components: Vec<String>,
  method: String,
  uri: String,
  http_version: HttpVersion,
}
