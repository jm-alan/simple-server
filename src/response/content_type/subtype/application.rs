#[derive(Debug, Default, Clone, Copy)]
pub enum ApplicationSubtype {
  #[default]
  OctetStream,
  Json,
  Wasm,
}
