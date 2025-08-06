use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use native_tls::TlsStream;
use websocket::sync::Client;
use websocket::OwnedMessage;

pub const TIMEOUT: u64 = 3000;

#[macro_export]
macro_rules! trace {
  ($l: literal) => {
    if cfg!(trace) {
      println!("DEBUG:{:?}", $l);
    }
  };
  ($l: literal, $e: expr) => {
    if cfg!(trace) {
      println!("DEBUG:{:?}-{:?}", $l, $e);
    }
  };
}
#[macro_export]
macro_rules! debug {
  ($l: literal) => {
    if cfg!(debug) {
      println!("DEBUG:{:?}", $l);
    }
  };
  ($l: literal, $e: expr) => {
    if cfg!(debug) {
      println!("DEBUG:{:?}-{:?}", $l, $e);
    }
  };
}

#[macro_export]
macro_rules! edebug {
  ($l: literal) => {
    if cfg!(debug) {
      eprintln!("DEBUG:{:?}", $l);
    }
  };
  ($l: literal, $e: expr) => {
    if cfg!(debug) {
      eprintln!("DEBUG:{:?}-{:?}", $l, $e);
    }
  };
}

macro_rules! shutdown {
  ($sock: ident, $l:literal, $e: expr) => {
    edebug!($l, $e);
    if let Err(e) = $sock.shutdown(Shutdown::Both) {
      edebug!("error shutting down socket:", e);
    }
  };
  ($sock: ident, $l:literal) => {
    edebug!($l);
    if let Err(e) = $sock.shutdown(Shutdown::Both) {
      edebug!("error shutting down socket:", e);
    }
  };
}

pub fn rcopy(stream: &mut TcpStream, client: Arc<Mutex<Client<TlsStream<TcpStream>>>>) {
  loop {
    let message = match client.lock().unwrap().recv_message() {
      Ok(v) => v,
      Err(e) => {
        shutdown!(stream, "error unwrapping incoming message from rcopy:", e);
        if let Err(e) = client.lock().unwrap().shutdown() {
          edebug!("error shutting down client:", e);
        }
        return;
      }
    };
    match message {
      OwnedMessage::Close(_) => {
        shutdown!(stream, "copied Client disconnected");
        if let Err(e) = client.lock().unwrap().shutdown() {
          edebug!("error shutting down client:", e);
        }
        return;
      }
      OwnedMessage::Binary(data_vec) => {
        if let Err(e) = stream.write(&data_vec.into_boxed_slice()) {
          shutdown!(stream, "error sending data_vec:", e);
          if let Err(e) = client.lock().unwrap().shutdown() {
            edebug!("error shutting down client:", e);
          }
          return;
        }
      }
      _ => {
        shutdown!(stream, "recv'd something that wasn't binary");
        if let Err(e) = client.lock().unwrap().shutdown() {
          edebug!("error shutting down client:", e);
        }
        return;
      }
    }
    thread::sleep(Duration::from_millis(1));
  }
}

pub fn wcopy(stream: &mut TcpStream, client: Arc<Mutex<Client<TlsStream<TcpStream>>>>) {
  loop {
    let mut buf = [0u8; 2048];
    match stream.peek(&mut buf) {
      Ok(0) => {
        if let Err(e) = client
          .lock()
          .unwrap()
          .send_message(&OwnedMessage::Close(None))
        {
          edebug!("Error sending message in wcopy:", e);
        }
        shutdown!(stream, "reading tcp socket has died");
        if let Err(e) = client.lock().unwrap().shutdown() {
          edebug!("error shutting down client:", e);
        }
        return;
      }
      Ok(n) => {
        let mut recv_vec = vec![0u8; n];
        if let Err(e) = stream.read_exact(&mut recv_vec) {
          if let Err(e) = client
            .lock()
            .unwrap()
            .send_message(&OwnedMessage::Close(None))
          {
            edebug!("Error sending close message in wcopy:", e);
          }
          shutdown!(stream, "error reading exact: ,closing thing", e);
          if let Err(e) = client.lock().unwrap().shutdown() {
            edebug!("error shutting down client:", e);
          }
          return;
        }

        let message = OwnedMessage::Binary(recv_vec);
        if let Err(e) = client.lock().unwrap().send_message(&message) {
          if let Err(e) = client
            .lock()
            .unwrap()
            .send_message(&OwnedMessage::Close(None))
          {
            edebug!("Error sending close message in wcopy:", e);
          }
          shutdown!(stream, "err sending message in wcopy:", e);
          if let Err(e) = client.lock().unwrap().shutdown() {
            edebug!("error shutting down client:", e);
          }
          return;
        }
      }
      Err(e) => {
        if let Err(e) = client
          .lock()
          .unwrap()
          .send_message(&OwnedMessage::Close(None))
        {
          edebug!("error sending the close in wcopy:", e);
        }
        shutdown!(stream, "err w/ tcpsocket, closing:", e);
        if let Err(e) = client.lock().unwrap().shutdown() {
          edebug!("error shutting down client:", e);
        }
        return;
      }
    }
    thread::sleep(Duration::from_millis(1));
  }
}
