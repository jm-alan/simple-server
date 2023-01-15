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
  Font(FontSubtype),
  Model(ModelSubtype),
  Example(ExampleSubtype),
}

impl From<ContentType> for String {
  fn from(content_type: ContentType) -> Self {
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
      ContentType::Font(font_subtype) => {
        format!("font/{}", font_subtype.to_string()).to_string()
      }
      ContentType::Model(model_subtype) => {
        format!("model/{}", model_subtype.to_string()).to_string()
      }
      ContentType::Example(example_subtype) => {
        format!("example/{}", example_subtype.to_string()).to_string()
      }
    }
  }
}

impl ToString for ContentType {
  fn to_string(&self) -> String {
    self.clone().into()
  }
}
