// extern crate websocket;

use chrono::{DateTime, Local};
use native_tls::{Identity, TlsAcceptor};
use std::fs::File;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use structopt::StructOpt;
use utils::{edebug, rcopy, wcopy, TIMEOUT};
use websocket::sync::{Client, Server};
use websocket::OwnedMessage;

#[derive(Debug, StructOpt)]
struct Opt {
  /// Socket address for front end proxy, the one to which proxychains might connect
  #[structopt(short)]
  frontend: String,

  /// Socket address for front end proxy, the one to which the client (implant) will connect
  #[structopt(short)]
  backend: String,
}

fn load_tls_acceptor() -> Result<TlsAcceptor, Box<dyn std::error::Error>> {
  let mut cert_file = File::open("./ssl/cert.pem")?;
  let mut key_file = File::open("./ssl/key.pem")?;
  let mut cert_data = Vec::new();
  let mut key_data = Vec::new();
  cert_file.read_to_end(&mut cert_data)?;
  key_file.read_to_end(&mut key_data)?;
  let mut identity = Vec::new();
  identity.extend_from_slice(&cert_data);
  identity.extend_from_slice(&key_data);
  let identity = Identity::from_pkcs8(&cert_data, &key_data)?;
  Ok(TlsAcceptor::new(identity)?)
}

#[tokio::main]
async fn main() {
  let args = Opt::from_args();

  let frontend = args.frontend;
  let backend = args.backend;

  let (front_t, front_r) = channel();

  thread::spawn(move || loop {
    let listener = match TcpListener::bind(&frontend) {
      Ok(l) => l,
      Err(e) => {
        eprintln!("err rebinding: {:?}", e);
        thread::sleep(Duration::from_millis(TIMEOUT));
        continue;
      }
    };
    thread::sleep(Duration::from_millis(1));
    let datetime: DateTime<Local> = SystemTime::now().into();
    match listener.accept() {
      Ok((fstream, addr)) => {
        println!(
          "{}|frontend connection|{}",
          datetime.format("%m-%d-%y|%T"),
          addr
        );
        if let Err(e) = front_t.send((fstream, addr)) {
          edebug!("error sending frontend sock to channel", e);
        }
      }
      _ => (),
    }
  });

  let acceptor = match load_tls_acceptor() {
    Ok(acc) => acc,
    Err(e) => {
      eprintln!("Failed to load TLS acceptor: {:?}", e);
      return;
    }
  };

  loop {
    let server = match Server::bind_secure(&backend, acceptor.clone()) {
      Ok(s) => s,
      Err(e) => {
        eprintln!("Failed to bind secure server: {:?}", e);
        thread::sleep(Duration::from_millis(TIMEOUT));
        continue;
      }
    };

    // New request from client socket (-b)
    for request in server.filter_map(Result::ok) {
      // Check for pending front-end socket
      match front_r.recv_timeout(Duration::from_millis(TIMEOUT)) {
        Ok((mut fstream, _addr)) => {
          thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
              request.reject().unwrap();
              return;
            }
            match request.use_protocol("rust-websocket").accept() {
              Ok(mut client) => {
                let ip = client.peer_addr().unwrap();
                let datetime: DateTime<Local> = SystemTime::now().into();
                println!(
                  "{}|proxy connection  |{}",
                  datetime.format("%m-%d-%y|%T"),
                  ip
                );
                handle_streams(&mut fstream, Arc::new(Mutex::new(client)));
              }
              Err(e) => {
                eprintln!("error with receiving client:{:?}", e);
              }
            }
            thread::sleep(Duration::from_millis(1));
          });
        } // OK

        Err(_) => {
          let datetime: DateTime<Local> = SystemTime::now().into();
          println!(
            "{}|no current connection|closing",
            datetime.format("%m-%d-%y|%T")
          );
          match request.use_protocol("rust-websocket").accept() {
            Ok(mut client) => {
              if let Err(e) = client.send_message(&OwnedMessage::Close(None)) {
                edebug!("error sending close message", e);
              }
              if let Err(e) = client.shutdown() {
                edebug!("error shutting down client:", e);
              }
            }
            Err(e) => {
              eprintln!("error with receiving client:{:?}", e);
            }
          }
        }
      }
    }
  }
}

fn handle_streams(
  fstream: &mut TcpStream,
  client: Arc<Mutex<Client<native_tls::TlsStream<TcpStream>>>>,
) {
  let mut inbound_in = match fstream.try_clone() {
    Ok(s) => s,
    Err(e) => {
      edebug!("error cloning socks", e);
      if let Err(e) = fstream.shutdown(Shutdown::Both) {
        edebug!("error sending closing fstream", e);
      }
      let mut client = client.lock().unwrap();
      if let Err(e) = client.send_message(&OwnedMessage::Close(None)) {
        edebug!("error sending close message", e);
      }
      if let Err(e) = client.shutdown() {
        edebug!("error shutting down client", e);
      }
      return;
    }
  };
  let mut inbound_out = match fstream.try_clone() {
    Ok(s) => s,
    Err(e) => {
      edebug!("error cloning socks", e);
      if let Err(e) = fstream.shutdown(Shutdown::Both) {
        edebug!("error sending closing fstream", e);
      }
      let mut client = client.lock().unwrap();
      if let Err(e) = client.send_message(&OwnedMessage::Close(None)) {
        edebug!("error sending close message", e);
      }
      if let Err(e) = client.shutdown() {
        edebug!("error shutting down client", e);
      }
      return;
    }
  };

  let client_read = Arc::clone(&client);
  let client_write = Arc::clone(&client);

  thread::spawn(move || {
    wcopy(&mut inbound_out, client_write);
  });

  thread::spawn(move || {
    rcopy(&mut inbound_in, client_read);
  });
}
