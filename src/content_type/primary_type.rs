#[derive(Debug, Default, Clone, Copy)]
pub enum ContentType {
  #[default]
  None,
  Application,
  Text,
  Audio,
  Font,
  Image,
  Model,
  Video,
  Example,
}
