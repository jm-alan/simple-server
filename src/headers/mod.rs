use std::collections::HashMap;

use super::content_type::ContentType;

#[derive(Debug, Default, Clone)]
pub struct Headers {
  pub content_type: ContentType,
  raw: HashMap<String, String>,
}
