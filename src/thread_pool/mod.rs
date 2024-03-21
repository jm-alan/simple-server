use std::{
  io::Write,
  net::TcpStream,
  sync::{
    mpsc::{self, SendError},
    Arc, Mutex,
  },
  thread,
};

use crate::{
  request::Request,
  response::{Response, StatusReason},
};

pub struct ThreadPool {
  size: usize,
  sender: mpsc::Sender<TcpStream>,
  handles: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
  pub fn new(size: usize) -> Self {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel::<TcpStream>();
    let arc_rec_guard = Arc::new(Mutex::new(receiver));

    let mut handles = Vec::with_capacity(size);

    for _ in 0..size {
      let cloned_rec = arc_rec_guard.clone();
      handles.push(thread::spawn(move || loop {
        let Ok(rec_guard) = cloned_rec.lock() else {
          continue;
        };

        let Ok(mut stream) = rec_guard.recv() else {
          continue;
        };

        drop(rec_guard);

        let Some(request) = Request::from_tcp(&stream) else {
          match stream
            .write_all(&Response::from(StatusReason::BadRequest).as_bytes())
          {
            Ok(_) => {
              println!("Successfully responded to bad request");
            }
            Err(err) => {
              println!(
                "Encountered error while responding to request {:#?}",
                err
              );
            }
          };
          return;
        };

        println!("{:#?}", request);

        match stream.write_all(&Response::from(StatusReason::Ok).as_bytes()) {
          Ok(_) => {
            println!("Successfully responded OK")
          }
          Err(err) => {
            println!(
              "Encountered error while responding to request {:#?}",
              err
            );
          }
        };
      }));
    }

    Self {
      size,
      sender,
      handles,
    }
  }

  pub fn process_stream(
    &self,
    stream: TcpStream,
  ) -> Result<(), SendError<TcpStream>> {
    self.sender.send(stream)
  }
}
