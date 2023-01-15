use super::TextSubtype;

impl From<&str> for TextSubtype {
  fn from(s: &str) -> Self {
    match s {
      "plain" => Self::Plain,
      "html" => Self::Html,
      "css" => Self::Css,
      "javascript" => Self::JavaScript,
      "csv" => Self::Csv,
      "calendar" => Self::Calendar,
      "xml" => Self::LegacyXml,
      _ => Self::Invalid,
    }
  }
}
