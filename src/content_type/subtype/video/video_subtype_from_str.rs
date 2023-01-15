use super::VideoSubtype;

impl From<&str> for VideoSubtype {
  fn from(s: &str) -> Self {
    match s {
      "x-msvideo" => Self::Avi,
      "mp4" => Self::Mp4,
      "mpeg" => Self::Mpeg,
      "ogg" => Self::Ogg,
      "mp2t" => Self::Mp2t,
      "webm" => Self::Webm,
      "3gpp" => Self::ThreeGpp,
      "3gpp2" => Self::ThreeGpp2,
      _ => Self::Unsupported,
    }
  }
}
