mod string_from_text_subtype;
mod text_subtype_from_str;

#[derive(Debug, Default, Clone)]
pub enum TextSubtype {
  #[default]
  Plain,
  Html,
  Css,
  JavaScript,
  Csv,
  Calendar,
  LegacyXml,
  Unsupported,
}
