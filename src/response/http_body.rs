use super::content_type::ContentType;

#[derive(Debug, Default, Clone)]
pub struct HttpBody {
  content_type: ContentType,
  raw_data: Box<u8>,
}

impl ToString for HttpBody {
  fn to_string(&self) -> String {
    match self.content_type {
      ContentType::None => "".to_string(),
      ContentType::Application(_) => todo!(),
      ContentType::Audio => todo!(),
      ContentType::Example => todo!(),
      ContentType::Font => todo!(),
      ContentType::Image => todo!(),
      ContentType::Model => todo!(),
      ContentType::Text => todo!(),
      ContentType::Video => todo!(),
    }
  }
}
