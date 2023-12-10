#[derive(Debug, Default, Clone, Copy)]
pub enum HttpMethod {
  #[default]
  GET,
  HEAD,
  POST,
  PUT,
  DELETE,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH,
}
