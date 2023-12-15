use crate::types::Headers;

#[inline(always)]
pub fn parse_headers(raw_bytes: &[u8]) -> Option<(Headers, Vec<u8>)> {
  let spliterator = raw_bytes.split(|byte| byte == &b'\n');

  let mut headers = vec![];
  let mut total_headers_length = 0;

  for maybe_header in
    spliterator.map(|byte_slice| (parse_header(byte_slice), byte_slice.len()))
  {
    let (Some(header), len) = maybe_header else {
      break;
    };

    headers.push(header);
    total_headers_length += len;
    total_headers_length += 1;
  }

  let inclusive_remainder =
    raw_bytes.get((total_headers_length + 1)..raw_bytes.len()).unwrap_or(&[]);

  let inclusive_remainder_empty_break =
    inclusive_remainder.iter().position(|byte| byte == &b'\n')?;

  Some((
    headers,
    inclusive_remainder
      .iter()
      .skip(inclusive_remainder_empty_break + 1)
      .copied()
      .collect(),
  ))
}

#[inline(always)]
fn parse_header(raw_bytes: &[u8]) -> Option<(String, String)> {
  if raw_bytes.is_empty() {
    return None;
  }

  let slice_len = raw_bytes.len();

  let mut key_start = 0;
  let mut key_end = 0;
  let mut val_start = slice_len;

  while key_end < val_start && raw_bytes[key_end] != b':' {
    key_end += 1
  }

  if key_end == val_start {
    return None;
  }

  val_start = key_end + 2;
  let mut val_end = val_start;

  while !((raw_bytes[key_start] >= b'A' && raw_bytes[key_start] <= b'Z')
    || (raw_bytes[key_start] >= b'a' && raw_bytes[key_start] <= b'z')
    || (raw_bytes[key_start] >= b'0' && raw_bytes[key_start] <= b'9')
    || raw_bytes[key_start] == b'-'
    || raw_bytes[key_start] == b'_')
    && key_start < key_end
  {
    key_start += 1
  }

  if key_start == key_end {
    return None;
  }

  while raw_bytes[val_end] != b'\r' && val_end < slice_len {
    val_end += 1
  }

  if val_start == val_end {
    return None;
  }

  let (Ok(key), Ok(val)) = (
    String::from_utf8(raw_bytes[key_start..key_end].into()),
    String::from_utf8(raw_bytes[val_start..val_end].into()),
  ) else {
    return None;
  };

  Some((key, val))
}
