use serialport::available_ports;
use std::thread::sleep;
use std::time::Duration;

use crate::input;
use crate::output;

pub fn manual() -> Option<String> {
    let mut available = available_ports().ok()?;

    output::print_ports(&available);

    let port = input::read_line();

    if port.to_lowercase().contains("dev/") || port.to_lowercase().contains("com") {
        Some(port)
    } else {
        let index = port.parse().ok()?;

        if index < available.len() {
            Some(available.remove(index).port_name)
        } else {
            None
        }
    }
}

pub fn auto() -> Option<String> {
    if let Ok(mut ports) = available_ports() {
        output::print_plug_in();

        loop {
            sleep(Duration::from_millis(500));

            if let Ok(new_ports) = available_ports() {
                for path in &new_ports {
                    if !ports.contains(&path) {
                        return Some(path.port_name.clone());
                    }
                }

                ports = new_ports;
            }
        }
    } else {
        output::print_no_access();
    }

    None
}
