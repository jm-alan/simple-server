use crate::types::Headers;

use super::{Request, StartLine};

impl Request {
  pub fn builder() -> RequestBuilder {
    RequestBuilder::new()
  }
}

#[derive(Default, Debug)]
pub struct RequestBuilder {
  start_line: Option<StartLine>,
  headers: Option<Headers>,
  body: Option<String>,
}

impl RequestBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn start_line(&mut self, start_line: StartLine) -> &mut Self {
    self.start_line = Some(start_line);

    self
  }

  pub fn headers(&mut self, headers: Headers) -> &mut Self {
    self.headers = Some(headers);

    self
  }

  pub fn body(&mut self, body: String) -> &mut Self {
    self.body = Some(body);

    self
  }

  pub fn build(self) -> Option<Request> {
    let RequestBuilder {
      start_line,
      headers,
      body,
    } = self;

    let start_line = start_line?;
    let headers = headers.unwrap_or_default();

    Some(Request {
      start_line,
      headers,
      body,
    })
  }
}
