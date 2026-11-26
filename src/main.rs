#![allow(unused_imports)]

mod codec;
mod commands;
use std::io::{self, Read, Write};
use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};
use crate::{commands::response_builder::build_response, codec::decode::parse};
use crate::codec::resp_types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //bind server addr
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    //loop to handle client-conn
    while let Ok((mut stream, _socket)) = listener.accept().await {
        tokio::spawn(async move {
            //512 byte buffer to store inputs
            let mut buffer: [u8; 512] = [0; 512];
            loop {
                match stream.readable().await {
                    Ok(_) => {
                        //read input into buf
                        if let Ok(bytes_read) = stream.try_read(&mut buffer) {
                            if bytes_read == 0 { break }
                            let mut input  = &buffer[0..bytes_read];
                            match parse(&mut input) {
                                Ok(parsed) => {
                                    println!("{parsed:?}");
                                    match build_response(&parsed) {
                                        Ok(response) => {println!("{response:?}");},
                                        Err(e) => {
                                            println!("{e:?}");
                                        }
                                    }
                                },
                                Err(e) => { eprintln!("failed to parse input: {e}") }
                            }
                            // for byte in 0..bytes_read { print!("{} ", buffer[byte]); #[allow(unused_must_use)] io::stdout().flush(); }
                            stream.write_all(b"+PONG\r\n").await.unwrap();
                        }
                    },
                    Err(e) => {
                        eprintln!("socket error: {e}");
                    }
                }
            }
        });
    }

    Ok(())
}