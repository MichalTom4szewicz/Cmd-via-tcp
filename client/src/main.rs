use std::io::prelude::*;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use std::*;

fn proceed(mut stream: std::net::TcpStream) {

    let mut msg = String::new();
    io::stdin()
    .read_line(&mut msg)
    .expect("Failed to read line");

    let bytes = stream.write(&msg.as_bytes());

    println!("sent {} bytes", bytes.unwrap());


}

fn main() {
    // hide_console_window();
    let stream = TcpStream::connect("192.168.1.19:3001");

    match stream {
        Ok(tcp) => proceed(tcp),
        Err(err) => println!("buuuu {}", err)
    }
}