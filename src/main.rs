mod arg_parser;

use arg_parser::{Args, Parser};

fn main() {
    let args = Args::parse();
    println!("Attacking {} with {} sockets", args.target, args.socket_num);
}
