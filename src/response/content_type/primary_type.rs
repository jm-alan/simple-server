use super::subtype::ApplicationSubtype;

#[derive(Debug, Default, Clone, Copy)]
pub enum ContentType {
  #[default]
  None,
  Application(ApplicationSubtype),
  Audio,
  Example,
  Font,
  Image,
  Model,
  Text,
  Video,
}
