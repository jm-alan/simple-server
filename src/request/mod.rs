use crate::{
  http_method::HttpMethod, http_version::HttpVersion, types::Headers,
};

mod builder;
mod request_from_tcp_stream;
mod start_line;

#[derive(Debug, Clone)]
pub struct Request {
  pub method: HttpMethod,
  pub uri: String,
  pub http_version: HttpVersion,
  pub headers: Option<Headers>,
  pub body: Option<String>,
}
