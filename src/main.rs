mod arg_parser;
mod attacker;

use arg_parser::{Args, Parser};

fn main() {
    let args = Args::parse();
    println!("[*] Resolving address");
    let addr = match attacker::resolve_address(args.target) {
        Ok(a) => a,
        Err(_) => {
            println!("[!] Error resolving address"); 
            std::process::exit(1);
        },
    };
    println!("[*] Creating {} connections", args.socket_num);
    let streams = attacker::make_connections(addr, args.port, args.socket_num);
    match streams {
        Ok(_) => println!("[*] Connections successful"),
        Err(e) => {
            println!("[!] Errors have occurred:");
            for i in e {
                println!("[!] ---- {}", i);
            }
        }
    };
}
