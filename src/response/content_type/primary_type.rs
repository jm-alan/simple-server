#[derive(Debug, Default, Clone, Copy)]
pub enum ContentType {
  #[default]
  None,
  Application,
  Audio,
  Example,
  Font,
  Image,
  Model,
  Text,
  Video,
}
