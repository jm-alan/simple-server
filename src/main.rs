mod request;
mod response;

use env_loader::Environment;
use std::{io::Write, net::TcpListener};

use request::Request;
use response::Response;

fn main() {
  let env = Environment::new();
  let host: String = env.require("HOST");
  let port: i32 = env.require("PORT");
  let address_string = format!("{host}:{port}");
  let Ok(listener) = TcpListener::bind(&address_string) else {
    panic!("Failed to bind to {address_string}");
  };
  for cx_attempt in listener.incoming() {
    let Ok(mut stream) = cx_attempt else {
      println!("Connection failed");
      continue;
    };
    let request: Request = (&mut stream).into();
    let response = Response::default();
    match stream.write_all(&response.serialize()) {
      Ok(_) => println!("Successfully responded with {}", response.to_string()),
      Err(err) => println!(
        "While responding to request {:#?}, encountered error {:#?}",
        request, err
      ),
    };
  }
  println!("Hello, world!");
}
