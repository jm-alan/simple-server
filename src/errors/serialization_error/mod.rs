#[derive(Debug, Default, Clone, Copy)]
pub struct SerializationError<'error_life> {
  pub location: &'error_life str,
  pub source_type: &'error_life str,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DeserializationError<'error_life> {
  pub location: &'error_life str,
  pub target_type: &'error_life str,
}

pub type SerializationResult<'error_life, T> =
  Result<T, SerializationError<'error_life>>;

pub type DeserializationResult<'error_life, T> =
  Result<T, DeserializationError<'error_life>>;
