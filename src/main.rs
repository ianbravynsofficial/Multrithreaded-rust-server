//we need this types for stuff like reading files,creating a tcp listener, 
//multithreading and handling streams.
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::time::Duration;
use std::thread;

fn main() {
    //this listener is going to bind to local host on port 7878.
    //we're unwraping because this procedure returns a result.(Result<T,E>).
    //this is not correct error handling:TO-DO
   let listener =  TcpListener::bind("127.0.0.1:7868").unwrap();

     //the method incoming() returns a result too.
    for stream in listener.incoming() {
        
         //we unwrap the T out of Ok(T) for now 
         //to get the stream out of the result and ignore the
         //err variant
        let stream = stream.unwrap();
        thread::spawn(|| {
            println!("got con!");
            handle_connection(stream);
        });
        
    }
}
// this fn handles our stream by reading it to a 512 byte buffer.
//once we have the stream,we define the possible requests we're anticipating.
//we then assign a varying filename and status line based on the request we got.
//these will make up our response.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n" , "hello.html")
    } 
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(30));
         ("HTTP/1.1 200 OK\r\n\r\n" , "hello.html")
    }
        else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","error.html")
    };

    //reading our html page from the file
    let contents = fs::read_to_string(filename).unwrap();

    let response  = format!("{}{}", status_line,contents);

    //the write method on stream accepts bytes as its args
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();//prevents write from exiting before all
    //the bytes are written.
}

