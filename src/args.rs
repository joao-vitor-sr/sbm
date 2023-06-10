use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub port_path: PathBuf,

    #[arg(short, long, default_value_t = 9600)]
    pub bud_rate: u32,
}
