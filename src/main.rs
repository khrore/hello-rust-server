use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

mod my_thread;
use my_thread::ThreadPool;

fn main() {
    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address).unwrap();
    let thread_pool = ThreadPool::build(4).unwrap();
    println!("Server is starting at {address} address");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let responce = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(responce.as_bytes()).unwrap();
}
