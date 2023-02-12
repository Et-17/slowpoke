use std::net::{IpAddr, AddrParseError, TcpStream};
use std::io::{Error, Write, self};

pub fn resolve_address(address: String) -> Result<IpAddr, AddrParseError> {
    return address.parse();
}

pub fn make_connections(target: IpAddr, port: u16,  socket_num: u16) -> Result<Vec<TcpStream>, Vec<Error>> {
    let mut connections: Vec<TcpStream> = Vec::new();
    let mut errors: Vec<Error> = Vec::new();
    for _ in 0..socket_num {
        let connection = match target {
            IpAddr::V4(x) => TcpStream::connect((x, port)),
            IpAddr::V6(x) => TcpStream::connect((x, port)),
        };
        match connection {
            Ok(x) => connections.push(x),
            Err(x) => errors.push(x),
        }
    }
    if connections.is_empty() {
        return Err(errors);
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