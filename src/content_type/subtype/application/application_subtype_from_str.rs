use regex::Regex;

use super::ApplicationSubtype;

impl From<&str> for ApplicationSubtype {
  fn from(s: &str) -> Self {
    match s {
      "octet-stream" => Self::OctetStream,
      "json" => Self::Json,
      "ld+json" => Self::JsonLd,
      "wasm" => Self::Wasm,
      "zip" => Self::Zip,
      "x-7z-compressed" => Self::SevenZip,
      "x-bzip" => Self::BZip,
      "x-bzip2" => Self::BZip2,
      "gzip" => Self::GZip,
      "msword" => Self::Doc,
      "vnd.openxmlformats-officedocument.wordprocessingml.document" => {
        Self::DocX
      }
      "vnd.ms-excel" => Self::Xls,
      "vnd.openxmlformats-officedocument.spreadsheetml.sheet" => Self::XlsX,
      "vnd.ms-powerpoint" => Self::Ppt,
      "vnd.openxmlformats-officedocument.presentationml.presentation" => {
        Self::PptX
      }
      "java-archive" => Self::Jar,
      "vnd.apple.installer+xml" => Self::AppleInstaller,
      "vnd.oasis.opendocument.text" => Self::ODText,
      "vnd.oasis.opendocument.spreadsheet" => Self::ODSheet,
      "vnd.oasis.opendocument.presentation" => Self::ODPres,
      "ogg" => Self::Ogg,
      "pdf" => Self::Pdf,
      "vnd.rar" => Self::Rar,
      "x-sh" => Self::Shell,
      "rtf" => Self::Rtf,
      "x-tar" => Self::Tar,
      "xml" => Self::Xml,
      "xhtml+xml" => Self::XHtml,
      subtype => {
        let raw_pattern =
          r#"^[^\u0000-\u0008\u000A-\u001F\u007F\(\)\[\]\{\}\\:@<>?"]$"#;
        let Ok(pattern) = Regex::new(raw_pattern) else {
          panic!("Failed to create regex pattern from rawstring literal: {raw_pattern}");
        };
        if pattern.is_match(subtype) {
          Self::Other(subtype.to_string())
        } else {
          Self::Invalid
        }
      }
    }
  }
}
