use super::subtype::ApplicationSubtype;

#[derive(Debug, Default, Clone)]
pub enum ContentType {
  #[default]
  None,
  Application(ApplicationSubtype),
  Text,
  Audio,
  Font,
  Image,
  Model,
  Video,
  Example,
}
