use std::{
    fs::{self, File},
    path::Path,
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use args::Args;
use clap::Parser;
use config::{parse_json_file, BalancaConfig};
use env_logger::Builder;
use log::{error, info};

use crate::port::connect_to_port;

mod args;
mod config;
mod port;

fn config_logger(config: &BalancaConfig) -> Result<()> {
    let target = Box::new(File::create(&config.balanca_log)?);

    Builder::new()
        .default_format()
        .target(env_logger::Target::Pipe(target))
        .init();

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = parse_json_file(&args.config)?;

    config_logger(&config)?;

    if !config.balanca_habilitar {
        error!("Balance is not enabled");
        return Err(anyhow!("Balance is not enabled"));
    }

    loop {
        if fs::metadata(&config.balanca_snd).is_ok() {
            info!(
                "File '{}' exists. Performing actions...",
                &config.balanca_snd.display()
            );
            match connect_to_port(
                &config.balanca_porta,
                config.balanca_velocidade,
                &config.balanca_rcv,
                &config.balanca_protocolo,
            ) {
                Err(e) => {
                    error!(
                        "Unable to execute the connection to the port: {} error: {}",
                        config.balanca_porta.display(),
                        e
                    );
                }
                Ok(_) => {}
            };
            delete_file(&config.balanca_snd)?;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn delete_file(file_path: &Path) -> Result<()> {
    fs::remove_file(file_path)?;
    info!("File '{}' deleted", file_path.display());
    Ok(())
}
