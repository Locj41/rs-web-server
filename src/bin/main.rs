
use std::time::Duration;
use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;    
use server::ThreadPool;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let pool = ThreadPool::new(4);


    for stream in listener.incoming() {
        let stream = stream.unwrap();
    
        pool.excute(||{
            hanle_connection(stream);
        });
    }
    println!("server is shutting down..");

}


fn hanle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // );
    
    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = 
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        }else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        }else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\nAgent: {}\r\n\r\n{}",
        status_line,
        content.len(),
        "Rust server",
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();



}