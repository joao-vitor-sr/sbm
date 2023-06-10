use std::path::Path;

use anyhow::{anyhow, Result};
use log::{error, info};
use serialport::SerialPort;

type SerialPortBox = Box<dyn SerialPort>;

pub fn return_port_connection(path: &Path, bud_rate: u32) -> Result<SerialPortBox> {
    let path = path.as_os_str().to_str();
    if let None = path {
        error!(
            "The path ({:#?}) for the path is invalid, please give other",
            path
        );
        return Err(anyhow!("Invalid path for the port"));
    }
    let path = path.unwrap();

    let port = serialport::new(path, bud_rate).open()?;

    Ok(port)
}

pub fn send_command(port: &mut SerialPortBox, command: &[u8; 1]) -> Result<()> {
    info!("Sending the following command {:#?} to the port", command);
    port.write(command)?;

    Ok(())
}
