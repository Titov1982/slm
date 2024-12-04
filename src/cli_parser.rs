use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Daemon "slmd" on/off
    #[arg(short = 'd', long = "daemon", default_value = "false")]
    pub daemon_on: bool,
    /// The path to the file to read CPU usage and MEM used
    #[arg(short = 'p', long = "file-path", default_value = "/tmp/daemon.dat")]
    pub path: std::path::PathBuf,
    /// The tick-rate (ms) for update receive data and interface
    #[arg(short = 't', long = "tick-rate", default_value = "1000")]
    pub tick_rate: u64,
}