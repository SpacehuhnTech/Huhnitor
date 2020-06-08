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

fn manual_port(port: String, ports: &mut Vec<SerialPortInfo>) -> Option<String> {
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

pub async fn manual(
    receiver: &mut UnboundedReceiver<String>,
    out: &output::Preferences,
) -> Option<String> {
    let mut ports = available_ports().ok()?;

    out.ports(&ports);

    let port = input::read_line(receiver).await?;

    manual_port(port, &mut ports)
}

pub async fn auto(
    receiver: &mut UnboundedReceiver<String>,
    out: &output::Preferences,
) -> Option<String> {
    let mut ports = available_ports().ok()?;

    out.ports(&ports);
    out.println("> Plug your deauther in, or type the port ID or name");

    tokio::select! {
        port = detect_port(&mut ports) => port,

        Some(port) = input::read_line(receiver) => {
            manual_port(port, &mut ports)
        }
    }
}
