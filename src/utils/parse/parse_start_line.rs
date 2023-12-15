#[inline(always)]
pub fn parse_start_line(raw_bytes: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
  let newline_begin =
    raw_bytes.iter().position(|byte| byte == &b'\r' || byte == &b'\n')?;

  let newline_end = raw_bytes.iter().position(|byte| byte == &b'\n')? + 1;

  let (start_line, inclusive_remainder) = raw_bytes.split_at(newline_begin);
  let proper_remainder = inclusive_remainder
    .get((newline_end - newline_begin)..inclusive_remainder.len())?;

  Some((start_line.into(), proper_remainder.into()))
}
