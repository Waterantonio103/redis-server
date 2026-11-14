#![allow(unused_imports)]
use std::{io::{Read, Write}};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    while let Ok((mut stream, _socket)) = listener.accept().await {
        tokio::spawn(async move {
            let mut buffer: [u8; 512] = [0; 512];
            loop {
                match stream.readable().await {
                    Ok(_) => {
                        if let Ok(bytes_read) = stream.try_read(&mut buffer) {
                            if bytes_read == 0 { break }
                            stream.write_all(b"+PONG\r\n").await.unwrap();
                        }
                    },
                    Err(e) => {
                        eprintln!("from outer: {e}");
                    }
                }
            }
        });
    }

    Ok(())
}