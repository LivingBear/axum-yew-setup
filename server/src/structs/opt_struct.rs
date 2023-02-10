use clap::Parser;
// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "Logos Server!")]
pub struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    pub log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    pub addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    pub static_dir: String,
}