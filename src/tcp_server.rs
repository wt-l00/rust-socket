use std::io;
use std::net::TcpListener;
use std::thread;
use std::str;
use std::io::{Read, Write};

pub fn serve(address: &str) -> io::Result<()> {
    
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                println!("An error occured while accepting a connection: {}", e);
                continue;
            }
        };
        
        thread::spawn(move || -> io::Result<()> {
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer)?;
                if nbytes == 0 {
                    return Ok(());
                }
                stream.write(&buffer[0..nbytes])?;
                return Ok(());
            }
        });
    }
    Ok(())
}

