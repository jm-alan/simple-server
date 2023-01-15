mod string_from_video_subtype;
mod video_subtype_from_str;

#[derive(Debug, Clone, Copy)]
pub enum VideoSubtype {
  Avi,
  Mp4,
  Mpeg,
  Ogg,
  Mp2t,
  Webm,
  ThreeGpp,
  ThreeGpp2,
  Unsupported,
}
