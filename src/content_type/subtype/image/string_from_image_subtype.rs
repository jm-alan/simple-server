use super::ImageSubtype;

impl From<ImageSubtype> for String {
  fn from(subtype: ImageSubtype) -> Self {
    match subtype {
      ImageSubtype::Avif => "avif".to_string(),
      ImageSubtype::Bmp => "bmp".to_string(),
      ImageSubtype::Gif => "gif".to_string(),
      ImageSubtype::Icon => "vnd.microsoft.icon".to_string(),
      ImageSubtype::Jpeg => "jpeg".to_string(),
      ImageSubtype::Png => "png".to_string(),
      ImageSubtype::Svg => "svg+xml".to_string(),
      ImageSubtype::Tiff => "tiff".to_string(),
      ImageSubtype::Webp => "webp".to_string(),
      ImageSubtype::Unsupported => {
        panic!("Attempt to stringify unsupported image subtype")
      }
    }
  }
}

impl ToString for ImageSubtype {
  fn to_string(&self) -> String {
    (*self).into()
  }
}
