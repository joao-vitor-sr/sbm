use std::{
    io::{self, Write},
    path::Path,
    time::{Duration, Instant}, fs,
};

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

    let port = serialport::new(path, bud_rate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    Ok(port)
}

pub fn send_command(port: &mut SerialPortBox, command: &[u8; 1]) -> Result<()> {
    info!("Sending the following command {:#?} to the port", command);
    port.write(command)?;

    Ok(())
}

fn execute_read_loop(port: &mut SerialPortBox, duration: Duration) -> Result<Vec<u8>> {
    let start_time = Instant::now();
    let mut buffer: Vec<u8> = Vec::new();

    loop {
        let mut read_buffer: [u8; 1] = [0; 1];
        if start_time.elapsed() >= duration {
            break;
        }

        match port.read(&mut read_buffer) {
            Ok(_) => {
                buffer.push(read_buffer[0]);
                if read_buffer[0] == 0x03 && valid_port_answer(&buffer) {
                    break;
                } else if read_buffer[0] == 0x03 {
                    buffer.clear();
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                info!("Response from the balance is timedOut");
                continue;
            }
            Err(_) => {
                return Err(anyhow!("Error on the answer in the balance"));
            }
        }
    }

    Ok(buffer)
}

fn treat_weight(values: &[u8]) -> Result<()> {
    let value = String::from_utf8_lossy(&values[1..values.len() - 1]);
    let mut file = fs::File::create("/tmp/.balanca_rcv")?;
    file.write_all(value.as_bytes())?;

    println!("{}", value);
    Ok(())
}

pub fn connect_to_port(port_path: &Path, bud_rate: u32) -> Result<()> {
    let mut port = return_port_connection(port_path, bud_rate)?;

    send_command(&mut port, &[0x05])?;

    let buffer = execute_read_loop(&mut port, Duration::from_secs(5))?;

    if !valid_port_answer(&buffer) {
        return Err(anyhow!("Invalid answer"));
    }
    treat_weight(&buffer)?;

    Ok(())
}

fn valid_port_answer(answer: &[u8]) -> bool {
    if answer.is_empty() || answer.len() < 7 {
        return false;
    }

    if answer[0] != 0x02 {
        return false;
    }

    let invalid_values: [u8; 3] = [0x53, 0x49, 0x4E];
    let valid = !answer[1..answer.len() - 1]
        .iter()
        .all(|item| invalid_values.contains(item));

    valid
}
