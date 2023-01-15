use super::AudioSubtype;

impl From<&str> for AudioSubtype {
  fn from(s: &str) -> Self {
    match s {
      "aac" => Self::Aac,
      "midi" | "x-midi" => Self::Midi,
      "mpeg" => Self::Mpeg,
      "ogg" => Self::Ogg,
      "opus" => Self::Opus,
      "wav" => Self::Wav,
      "webm" => Self::Webm,
      "3gpp" => Self::ThreeGpp,
      "3gpp2" => Self::ThreeGpp2,
      _ => Self::Invalid,
    }
  }
}
