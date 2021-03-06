use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

/**
 * connect specific port and ip adress
 */

pub fn connect(adress: &str) -> Result<(), failure::Error> {
    let mut stream = TcpStream::connect(adress)?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        println!("{}", str::from_utf8(&buffer)?);
    }
    Ok(())
}
