use crate::content_type::ContentType;

#[derive(Debug, Default, Clone)]
pub struct HttpBody {
  content_type: ContentType,
  raw_data: Box<u8>,
}

impl ToString for HttpBody {
  fn to_string(&self) -> String {
    todo!()
  }
}
