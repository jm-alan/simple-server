use std::{
  io::{BufRead, BufReader},
  net::TcpStream,
};

use crate::utils::parse::{parse_body, parse_headers, parse_start_line};

use super::{Request, StartLine};

impl Request {
  #[inline(always)]
  pub fn from_tcp(stream: &TcpStream) -> Option<Self> {
    let mut reader = BufReader::with_capacity(16384, stream);

    let headers_max = reader.fill_buf().ok()?;
    let (raw_start_line, maybe_headers) = parse_start_line(headers_max)?;
    let headers_and_remainder = parse_headers(&maybe_headers)?;
    let start_line = StartLine::from_bytes(&raw_start_line)?;

    let headers = headers_and_remainder.0;

    let body = match (
      start_line.method.permits_body(),
      headers.iter().find(|(key, _)| {
        key == "content-length"
          || key == "Content-Length"
          || key == "CONTENT-LENGTH"
      }),
    ) {
      (false, _) | (_, None) => None,
      (_, Some((_, raw_content_length))) => {
        let init = headers_and_remainder.1;
        let consumed_len = headers_max.len();
        reader.consume(consumed_len);

        parse_body(init, raw_content_length, &mut reader)
      }
    };

    Some(Self {
      start_line,
      headers,
      body,
    })
  }
}
