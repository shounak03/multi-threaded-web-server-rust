
use std::time::Duration;
use std::{fs, thread};
use std::net::{TcpListener,TcpStream}; 
use std::io::{prelude::*};

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

    /*
    Steps for handling a connection


    1-> BufReader wraps the stream to provide buffered  reading. Instead of reading byte-by-byte from the TCP stream, it reads chunks into memory for more efficient processing.

    2-> buf_reader.lines()
    Returns an iterator over the lines of the stream. Each item is a Result<String, std::io::Error>.

    3-> .map(|result| result.unwrap())
    Converts the iterator from Result<String, Error> to String by unwrapping the result.

    4->.take_while(|line| !line.is_empty())
    Continues reading lines as long as they are not empty.

    In HTTP, an empty line ("") indicates the end of the request headers, so this captures only the request line and headers.

    5->.collect()
    Collects the lines into a Vec<String> and stores them in http_request.

    6-> creating a response

    7-> we call as_bytes on our response to convert the string data to bytes. The write_all method on stream takes a &[u8] and sends those bytes directly down the connection.


*/


    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_id = b"GET /id HTTP/1.1\r\n";

    let (status, filename) = 
        if buffer.starts_with(get){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK" ,"frontend/index.html")
        }else  if buffer.starts_with(get_id){
            ("HTTP/1.1 200 OK","frontend/Id.html")
        }else{
            ("HTTP/1.1 404 NOT FOUND","frontend/404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();

    //response format
    // {status_line}\r\nContent-Length: {length}\r\n\r\n{contents}

    let response = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );



    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
     
}