pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Et-17")]
#[command(about = "An improved Slowloris implementation in Rust")]
#[command(long_about = None)]
pub struct Args {
    /// Amount of socket connections to open
    #[arg(short, long, default_value_t = 200)]
    pub socket_num: u16,

    #[arg(short, long, default_value_t = 80)]
    pub port: u16,

    /// Address of the target
    #[arg(short, long)]
    pub target: String,

    /// Use randomized headers
    #[arg(short, long, default_value_t = false)]
    pub randheaders: bool,

}
