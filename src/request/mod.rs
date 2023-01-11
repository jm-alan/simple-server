mod request_from_tcp_stream;

#[derive(Debug, Default, Clone)]
pub struct Request {
  raw_components: Vec<String>,
}
