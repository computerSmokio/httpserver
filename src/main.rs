use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::string::FromUtf8Error;

fn listen_stream(mut stream: TcpStream) -> std::result::Result<String, FromUtf8Error>{
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    let chain = String::from_utf8(buffer);
    println!("{:?}", chain);
    Ok(chain?)
}

fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    for stream in listener.incoming(){
        let chain = listen_stream(stream?).unwrap();
        println!("{}", chain);
    }
    Ok(())
}