use super::subtype::*;

#[derive(Debug, Default, Clone)]
pub enum ContentType {
  #[default]
  None,
  Application(ApplicationSubtype),
  Text(TextSubtype),
  Audio(AudioSubtype),
  Image(ImageSubtype),
  Video(VideoSubtype),
}

impl From<&ContentType> for String {
  #[inline(always)]
  fn from(content_type: &ContentType) -> Self {
    match content_type {
      ContentType::None => "".to_string(),
      ContentType::Application(subtype) => {
        format!("application/{}", subtype.to_string()).to_string()
      }
      ContentType::Text(text_subtype) => {
        format!("text/{}", text_subtype.to_string()).to_string()
      }
      ContentType::Audio(audio_subtype) => {
        format!("audio/{}", audio_subtype.to_string()).to_string()
      }
      ContentType::Image(image_subtype) => {
        format!("image/{}", image_subtype.to_string()).to_string()
      }
      ContentType::Video(video_subtype) => {
        format!("video/{}", video_subtype.to_string()).to_string()
      }
    }
  }
}

impl ToString for ContentType {
  #[inline(always)]
  fn to_string(&self) -> String {
    self.into()
  }
}
