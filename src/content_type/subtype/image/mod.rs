mod image_subtype_from_str;
mod string_from_image_subtype;

#[derive(Debug, Clone, Copy)]
pub enum ImageSubtype {
  Avif,
  Bmp,
  Gif,
  Jpeg,
  Png,
  Svg,
  Tiff,
  Webp,
  Icon,
  Unsupported,
}
