mod application_subtype_from_str;
mod string_from_application_subtype;

#[derive(Debug, Default, Clone)]
pub enum ApplicationSubtype {
  #[default]
  OctetStream,
  Json,
  JsonLd,
  Wasm,
  Zip,
  SevenZip,
  BZip,
  BZip2,
  GZip,
  Doc,
  DocX,
  Ppt,
  PptX,
  Xls,
  XlsX,
  Jar,
  AppleInstaller,
  ODText,
  ODSheet,
  ODPres,
  Ogg,
  Pdf,
  Rar,
  Rtf,
  Shell,
  Tar,
  Xml,
  XHtml,
  Invalid,
  Other(String),
}
