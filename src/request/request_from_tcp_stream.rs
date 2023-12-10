use std::{
  io::{BufRead, BufReader},
  net::TcpStream,
};

use crate::{http_method::HttpMethod, http_version::HttpVersion};

use super::Request;

impl From<&mut TcpStream> for Request {
  #[inline(always)]
  fn from(mut stream: &mut TcpStream) -> Self {
    let raw_components: Vec<String> = BufReader::new(&mut stream)
      .lines()
      .map(|buf_result| buf_result.unwrap_or("\r\n".into()))
      .take_while(|line| !line.is_empty())
      .collect();

    println!("{:#?}", raw_components);

    Self {
      raw_components,
      method: HttpMethod::GET,
      uri: "".to_string(),
      http_version: HttpVersion::Modern,
    }
  }
}
