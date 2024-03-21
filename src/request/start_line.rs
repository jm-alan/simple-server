use crate::{http_method::HttpMethod, http_version::HttpVersion};

#[derive(Debug, Clone)]
pub struct StartLine {
  pub method: HttpMethod,
  pub uri: String,
  pub http_version: HttpVersion,
}

impl StartLine {
  #[inline(always)]
  pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
    if bytes.is_empty() {
      return None;
    }

    let len = bytes.len();

    let mut method_end = 0;
    let mut version_start = len - 1;
    let mut version_end = len - 1;

    while method_end < len && bytes[method_end] != b' ' {
      method_end += 1;
    }

    while version_start > method_end && bytes[version_start] != b' ' {
      version_start -= 1;
    }

    while version_end > version_start
      && (bytes[version_end] == b'\r' || bytes[version_end] == b'\n')
    {
      version_end -= 1
    }

    if method_end == len
      || version_start == method_end
      || version_end == version_start
    {
      return None;
    }

    let (Some(method_slice), Some(uri_slice), Some(version_slice)) = (
      bytes.get(0..method_end),
      bytes.get((method_end + 1)..version_start),
      bytes.get((version_start + 1)..=version_end),
    ) else {
      return None;
    };

    let (Ok(method), Ok(uri), Ok(http_version)) = (
      std::str::from_utf8(method_slice),
      std::str::from_utf8(uri_slice),
      std::str::from_utf8(version_slice),
    ) else {
      return None;
    };

    Some(Self {
      method: method.into(),
      uri: uri.into(),
      http_version: http_version.into(),
    })
  }
}
