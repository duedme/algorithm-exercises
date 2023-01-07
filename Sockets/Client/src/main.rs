use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() -> std::io::Result<()> {
    let addr = "localhost:9099";
    let mut stream = TcpStream::connect(addr)?;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "adios" {
            stream.write_all(input.as_bytes())?;
            stream.shutdown(std::net::Shutdown::Both)?;
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buf = [0; 4096];
        let n = stream.read(&mut buf)?;
        let response = from_utf8(&buf[..n]).unwrap();
        println!("{}", response);
    }
    Ok(())
}
