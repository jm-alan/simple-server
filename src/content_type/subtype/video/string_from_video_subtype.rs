use super::VideoSubtype;

impl From<VideoSubtype> for String {
  fn from(subtype: VideoSubtype) -> Self {
    match subtype {
      VideoSubtype::Avi => "x-msvideo".to_string(),
      VideoSubtype::Mp4 => "mp4".to_string(),
      VideoSubtype::Mpeg => "mpeg".to_string(),
      VideoSubtype::Ogg => "ogg".to_string(),
      VideoSubtype::Mp2t => "mp2t".to_string(),
      VideoSubtype::Webm => "webm".to_string(),
      VideoSubtype::ThreeGpp => "3gpp".to_string(),
      VideoSubtype::ThreeGpp2 => "3gpp2".to_string(),
      VideoSubtype::Unsupported => {
        panic!("Attempt to stringify unsupported video subtype")
      }
    }
  }
}

impl ToString for VideoSubtype {
  fn to_string(&self) -> String {
    (*self).into()
  }
}
