use std::{
  io::Write,
  net::TcpStream,
  sync::{
    mpsc::{self, SendError},
    Arc, Mutex,
  },
  thread,
};

use crate::{request::Request, response::Response};

pub struct ThreadPool {
  size: usize,
  sender: mpsc::Sender<TcpStream>,
  threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
  pub fn new(size: usize) -> Self {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel::<TcpStream>();
    let arc_rec_guard = Arc::new(Mutex::new(receiver));

    let mut threads = Vec::with_capacity(size);

    for _ in 0..size {
      let cloned_rec = arc_rec_guard.clone();
      threads.push(thread::spawn(move || loop {
        let Ok(rec_guard) = cloned_rec.lock() else {
          continue;
        };

        let Ok(mut stream) = rec_guard.recv() else {
          continue;
        };

        drop(rec_guard);

        let request = Request::from_tcp(&stream);

        println!("{:#?}", request);

        match stream.write_all(&Response::default().as_bytes()) {
          Ok(_) => {
            // println!("Successfully responded with {}", response)
          }
          Err(_) => {
            // println!(
            //   "While responding to request {:#?}, encountered error {:#?}",
            //   request, err
            // );
          }
        };
      }));
    }

    Self {
      size,
      sender,
      threads,
    }
  }

  pub fn process_stream(
    &self,
    stream: TcpStream,
  ) -> Result<(), SendError<TcpStream>> {
    self.sender.send(stream)
  }
}
