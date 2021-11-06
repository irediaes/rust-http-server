use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Streams: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf[..]) {
                                Ok(_) => {}
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read buffer from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to connect top server: {}", e),
            }
        }
    }
}
