mod content_type;
mod errors;
mod http_method;
mod http_version;
mod json;
mod request;
mod response;
mod thread_pool;
mod traits;
mod types;
mod utils;

use env_loader::Environment;
use std::net::TcpListener;

use crate::thread_pool::ThreadPool;

fn main() {
  let env = Environment::new();

  let host: String = env.require("HOST");
  let port: i32 = env.require("PORT");
  let address_string = format!("{host}:{port}");
  let Ok(listener) = TcpListener::bind(&address_string) else {
    panic!("Failed to bind to {address_string}");
  };

  let pool = ThreadPool::new(6);

  println!("UP {address_string}");

  for cx_attempt in listener.incoming() {
    let Ok(stream) = cx_attempt else {
      println!("Connection failed");
      continue;
    };

    let Ok(_) = pool.process_stream(stream) else {
      continue;
    };
  }
}
