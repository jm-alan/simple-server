use super::ApplicationSubtype;

impl From<ApplicationSubtype> for String {
  fn from(subtype: ApplicationSubtype) -> Self {
    match subtype {
      ApplicationSubtype::OctetStream => "octet-stream".to_string(),
      ApplicationSubtype::Json => "json".to_string(),
      ApplicationSubtype::JsonLd => "ld+json".to_string(),
      ApplicationSubtype::Wasm => "wasm".to_string(),
      ApplicationSubtype::Zip => "zip".to_string(),
      ApplicationSubtype::SevenZip => "x-7z-compressed".to_string(),
      ApplicationSubtype::BZip => "x-bzip".to_string(),
      ApplicationSubtype::BZip2 => "x-bzip2".to_string(),
      ApplicationSubtype::GZip => "gzip".to_string(),
      ApplicationSubtype::Doc => "msword".to_string(),
      ApplicationSubtype::DocX => {
        "vnd.openxmlformats-officedocument.wordprocessingml.document"
          .to_string()
      }
      ApplicationSubtype::Xls => "vnd.ms-excel".to_string(),
      ApplicationSubtype::XlsX => {
        "vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()
      }
      ApplicationSubtype::Ppt => "vnd.ms-powerpoint".to_string(),
      ApplicationSubtype::PptX => {
        "vnd.openxmlformats-officedocument.presentationml.presentation"
          .to_string()
      }
      ApplicationSubtype::Jar => "java-archive".to_string(),
      ApplicationSubtype::AppleInstaller => {
        "vnd.apple.installer+xml".to_string()
      }
      ApplicationSubtype::ODText => "vnd.oasis.opendocument.text".to_string(),
      ApplicationSubtype::ODSheet => {
        "vnd.oasis.opendocument.spreadsheet".to_string()
      }
      ApplicationSubtype::ODPres => {
        "vnd.oasis.opendocument.presentation".to_string()
      }
      ApplicationSubtype::Ogg => "ogg".to_string(),
      ApplicationSubtype::Pdf => "pdf".to_string(),
      ApplicationSubtype::Rar => "vnd.rar".to_string(),
      ApplicationSubtype::Shell => "x-sh".to_string(),
      ApplicationSubtype::Rtf => "rtf".to_string(),
      ApplicationSubtype::Tar => "x-tar".to_string(),
      ApplicationSubtype::Xml => "xml".to_string(),
      ApplicationSubtype::XHtml => "xhtml+xml".to_string(),
      ApplicationSubtype::Other(uncommon_subtype) => uncommon_subtype,
      ApplicationSubtype::Invalid => {
        panic!("Attempt to stringify invalid application subtype")
      }
    }
  }
}

impl ToString for ApplicationSubtype {
  fn to_string(&self) -> String {
    self.clone().into()
  }
}
