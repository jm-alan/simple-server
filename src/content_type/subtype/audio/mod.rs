mod audio_subtype_from_str;
mod string_from_audio_subtype;

#[derive(Debug, Clone, Copy)]
pub enum AudioSubtype {
  Aac,
  Midi,
  Mpeg,
  Ogg,
  Opus,
  Wav,
  Webm,
  ThreeGpp,
  ThreeGpp2,
  Unsupported,
}
