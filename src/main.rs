use std::{fs, thread, time::Duration};

use anyhow::Result;
use args::Args;
use clap::Parser;
use log::info;

use crate::port::connect_to_port;

mod args;
mod port;

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let enq_send_file = "/tmp/.balanca_snd";

    loop {
        if fs::metadata(&enq_send_file).is_ok() {
            info!("File '{}' exists. Performing actions...", enq_send_file);
            connect_to_port(&args.port_path, args.bud_rate).unwrap_or(());
            delete_file(&enq_send_file)?;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn delete_file(file_path: &str) -> Result<()> {
    fs::remove_file(file_path)?;
    info!("File '{}' deleted", file_path);
    Ok(())
}
