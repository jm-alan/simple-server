use std::{
  io::{BufRead, BufReader},
  net::TcpStream,
};

pub fn parse_body(
  mut init: Vec<u8>,
  raw_content_length: &str,
  reader: &mut BufReader<&TcpStream>,
) -> Option<Vec<u8>> {
  let Ok(content_length) = raw_content_length.parse::<usize>() else {
    return None;
  };

  init.reserve_exact(init.len() - content_length);

  while init.len() < init.capacity() {
    let Ok(rolling_body) = reader.fill_buf() else {
      return None;
    };

    if rolling_body.is_empty() {
      break;
    }

    let rolling_len = rolling_body.len();

    init.extend_from_slice(rolling_body);

    reader.consume(rolling_len);
  }

  Some(init)
}
