use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub enum ApplicationSubtype {
  #[default]
  OctetStream,
  Json,
  Wasm,
  Other(String),
  Invalid,
}

impl FromStr for ApplicationSubtype {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "octet-stream" => Ok(Self::OctetStream),
      "application-json" => Ok(Self::OctetStream),
      "wasm" => Ok(Self::Wasm),
      subtype => {
        let raw_pattern =
          r#"^[^\u0000-\u0008\u000A-\u001F\u007F\(\)\[\]\{\}\\:@<>?"]$"#;
        let Ok(pattern) = Regex::new(raw_pattern) else {
          panic!("Failed to create regex pattern from rawstring literal: {raw_pattern}");
        };
        if pattern.is_match(subtype) {
          Ok(Self::Other(subtype.to_string()))
        } else {
          Err(format!("application/{subtype} is not a valid MIME type because it contains invalid characters"))
        }
      }
    }
  }
}
