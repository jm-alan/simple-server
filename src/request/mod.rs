use crate::types::Headers;

mod request_from_tcp_stream;
mod start_line;

pub use start_line::StartLine;

#[derive(Debug)]
pub struct Request {
  pub start_line: StartLine,
  pub headers: Headers,
  pub body: Option<Vec<u8>>,
}
