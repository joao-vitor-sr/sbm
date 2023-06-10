use anyhow::Result;
use args::Args;
use clap::Parser;
use port::{return_port_connection, send_command};

mod args;
mod port;

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let mut port = return_port_connection(&args.port_path, args.bud_rate)?;

    send_command(&mut port, &[0x05])?;

    let mut serial_buf: Vec<u8> = vec![0; 256];
    let result = port.read(&mut serial_buf)?;
    Ok(())
}
