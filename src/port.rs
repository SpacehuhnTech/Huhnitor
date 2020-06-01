use serialport::{available_ports, SerialPortInfo};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::input;
use crate::output;

async fn detect_port(ports: &mut Vec<SerialPortInfo>) -> Option<String> {
    loop {
        tokio::time::delay_for(std::time::Duration::from_millis(500)).await;

        if let Ok(new_ports) = available_ports() {
            for path in &new_ports {
                if !ports.contains(&path) {
                    return Some(path.port_name.clone());
                }
            }

            *ports = new_ports;
        }
    }
}

pub async fn manual(receiver: &mut UnboundedReceiver<String>) -> Option<String> {
    let mut ports = available_ports().ok()?;

    output::print_ports(&ports);

    let port = input::read_line(receiver).await?;

    if port.to_lowercase().contains("dev/") || port.to_lowercase().contains("com") {
        Some(port)
    } else {
        let index = port.parse().ok()?;

        if index < ports.len() {
            Some(ports.remove(index).port_name)
        } else {
            None
        }
    }
}

pub async fn auto(receiver: &mut UnboundedReceiver<String>) -> Option<String> {
    let mut ports = available_ports().ok()?;
    output::print_ports(&ports);
    output::print_plug_in();

    tokio::select! {
        port = detect_port(&mut ports) => port,

        Some(port) = input::read_line(receiver) => {
            if port.to_lowercase().contains("dev/") || port.to_lowercase().contains("com") {
                Some(port)
            } else {
                let index = port.parse().ok()?;

                if index < ports.len() {
                    Some(ports.remove(index).port_name)
                } else {
                    None
                }
            }
        }
    }
}
