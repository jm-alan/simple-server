use super::ImageSubtype;

impl From<&str> for ImageSubtype {
  fn from(s: &str) -> Self {
    match s {
      "avif" => Self::Avif,
      "bmp" => Self::Bmp,
      "gif" => Self::Gif,
      "vnd.microsoft.icon" => Self::Icon,
      "jpeg" => Self::Jpeg,
      "png" => Self::Png,
      "svg+xml" => Self::Svg,
      "tiff" => Self::Tiff,
      "webp" => Self::Webp,
      _ => Self::Unsupported,
    }
  }
}
