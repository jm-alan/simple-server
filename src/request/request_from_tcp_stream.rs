use std::{
  io::{BufRead, BufReader},
  net::TcpStream,
};

use super::Request;

impl From<&mut TcpStream> for Request {
  fn from(mut stream: &mut TcpStream) -> Self {
    Self {
      raw_components: BufReader::new(&mut stream)
        .lines()
        .map(|buffer_line| {
          if let Ok(line) = buffer_line {
            line
          } else {
            "".to_string()
          }
        })
        .take_while(|line| !line.is_empty())
        .collect(),
    }
  }
}
