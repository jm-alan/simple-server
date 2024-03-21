use crate::json::Json;

pub fn parse_json(raw_bytes: &[u8]) -> Option<Json> {
  let mut decomposed = None;

  let mut writing_key = false;
  let mut quoted = false;

  let mut buffer = Vec::new();
  let mut current_key = None;

  for byte in raw_bytes {
    match (byte, quoted, writing_key) {
      (b'"', false, _) => {
        quoted = true;
      }
      (b'"', true, key) => {
        writing_key = false;
        quoted = false;

        if key {
          let Ok(valid_str) = String::from_utf8(buffer.clone()) else {
            return None;
          };

          current_key = Some(valid_str);
        } else if let Some(extant_key) = current_key {
          write_value(&mut decomposed, extant_key, &buffer);
          current_key = None;
        } else {
          return parse_fragment(&buffer);
        }

        buffer.clear();
      }
      (b':', false, false) => {}
      (b',', false, false) => {}
      (b'[', false, false) => {}
      (b']', false, false) => {}
      (b'{', false, false) => {
        writing_key = true;
      }
      (b'}', false, false) => {}
      (ch, true, _) | (ch, false, false) => {
        buffer.push(*ch);
      }
      _ => return None,
    }
  }

  decomposed
}

fn write_value(container: &mut Option<Json>, key: String, val: &[u8]) {}
fn parse_fragment(fragment: &[u8]) -> Option<Json> {
  None
}
