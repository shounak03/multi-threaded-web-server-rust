
use std::time::Duration;
use std::{fs, thread};
use std::net::{TcpListener,TcpStream}; 
use std::io::prelude::*;

use server::ThreadPool;
fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listner.incoming(){
        let stream=stream.unwrap();

        pool.execute(|| {

            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream:TcpStream){

    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_id = b"GET /id HTTP/1.1\r\n";

    let (status, filename) = 
        if buffer.starts_with(get){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK" ,"frontend/index.html")
        }else  if buffer.starts_with(get_id){
            ("HTTP/1.1 404 NOT FOUND","frontend/Id.html")
        }else{
            ("HTTP/1.1 404 NOT FOUND","frontend/404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
     
}