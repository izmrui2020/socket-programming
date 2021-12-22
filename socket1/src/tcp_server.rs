use log::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

/**
 * keep recieving connection at specific addr.
 */

pub fn server(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

/**
 * keep recieving input from client. if get data from it, return same data.
 */

fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling dtafrom {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024]; //TODO: bufferについてもっと詳しくなろう。

    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        println!("{}", str::from_utf8(&buffer[..nbytes])?);
        stream.write_all(&buffer[..nbytes])?;
    }
}
