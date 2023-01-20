use std::io::{Read, Write};
use std::net::{TcpListener};
use std::str::from_utf8;

fn main() -> std::io::Result<()> {
    let addr = "localhost:9099";
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Conectado a un cliente: {}", stream.peer_addr()?);

        loop {
            let mut buf = [0; 4096];
            let n = stream.read(&mut buf)?;
            if n == 0 {
                println!("El cliente se ha desconectado.");
                break;
            }
            let msg = from_utf8(&buf[..n]).unwrap();
            println!("{}", msg);

            if msg == "bye" {
                break;
            }

            let mut response = String::new();
            std::io::stdin().read_line(&mut response)?;
            stream.write_all(response.as_bytes())?;
        }
    }
    Ok(())
}
