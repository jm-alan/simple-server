#[derive(Debug, Default, Clone)]
pub enum StatusReason {
  #[default]
  Ok,
  Created,
  BadRequest,
  Unauthorized,
  Forbidden,
  InternalServerError,
  NotImplemented,
  BadGateway,
  ServiceUnavailable,
}

impl ToString for StatusReason {
  fn to_string(&self) -> String {
    match self {
      Self::Ok => "200 OK".to_string(),
      Self::Created => "201 CREATED".to_string(),
      Self::BadRequest => "400 BAD REQUEST".to_string(),
      Self::Unauthorized => "401 UNAUTHORIZED".to_string(),
      Self::Forbidden => "403 FORBIDDEN".to_string(),
      Self::InternalServerError => "500 INTERNAL SERVER ERROR".to_string(),
      Self::NotImplemented => "501 NOT IMPLEMENTED".to_string(),
      Self::BadGateway => "502 BAD GATEWAY".to_string(),
      Self::ServiceUnavailable => "503 SERVICE UNAVAILABLE".to_string(),
    }
  }
}
