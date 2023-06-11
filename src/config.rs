use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct BalancaConfig {
    pub balanca_habilitar: bool,
    pub balanca_protocolo: String,
    pub balanca_porta: PathBuf,
    pub balanca_velocidade: u32,
    pub balanca_snd: PathBuf,
    pub balanca_rcv: PathBuf,
    pub balanca_log: PathBuf,
}

pub fn parse_json_file(file_path: &Path) -> Result<BalancaConfig> {
    let file_contents = std::fs::read_to_string(file_path)?;
    let config: BalancaConfig = serde_json::from_str(&file_contents)?;
    Ok(config)
}
