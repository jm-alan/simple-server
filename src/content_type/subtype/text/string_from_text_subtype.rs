use super::TextSubtype;

impl From<TextSubtype> for String {
  fn from(subtype: TextSubtype) -> Self {
    match subtype {
      TextSubtype::Plain => "plain".to_string(),
      TextSubtype::Html => "html".to_string(),
      TextSubtype::Css => "css".to_string(),
      TextSubtype::JavaScript => "javascript".to_string(),
      TextSubtype::Csv => "csv".to_string(),
      TextSubtype::Calendar => "calendar".to_string(),
      TextSubtype::LegacyXml => "xml".to_string(),
      TextSubtype::Unsupported => {
        panic!("Attempt to stringify unsupported text subtype")
      }
    }
  }
}

impl ToString for TextSubtype {
  fn to_string(&self) -> String {
    (*self).into()
  }
}
