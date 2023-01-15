use super::AudioSubtype;

impl From<AudioSubtype> for String {
  fn from(subtype: AudioSubtype) -> Self {
    match subtype {
      AudioSubtype::Aac => "aac".to_string(),
      AudioSubtype::Midi => "midi".to_string(),
      AudioSubtype::Mpeg => "mpeg".to_string(),
      AudioSubtype::Ogg => "ogg".to_string(),
      AudioSubtype::Opus => "opus".to_string(),
      AudioSubtype::Wav => "wav".to_string(),
      AudioSubtype::Webm => "webm".to_string(),
      AudioSubtype::ThreeGpp => "3gpp".to_string(),
      AudioSubtype::ThreeGpp2 => "3gpp2".to_string(),
      AudioSubtype::Invalid => {
        panic!("Attempt to stringify invalid audio subtype")
      }
    }
  }
}

impl ToString for AudioSubtype {
  fn to_string(&self) -> String {
    (*self).into()
  }
}
