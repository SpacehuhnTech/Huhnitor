use serialport::available_ports;
use std::thread::sleep;
use std::time::Duration;

use crate::input;
use crate::output;

pub fn manual(out: &output::Preferences) -> Option<String> {
    let available = available_ports().ok()?;

    out.ports(&available);

    let port = input::read_line();

    Some(port)
}

pub fn auto(out: &output::Preferences) -> Option<String> {
    if let Ok(original) = available_ports() {
        out.println("> Plug in your device");

        for _ in 0..30 {
            if let Ok(paths) = available_ports() {
                for path in paths {
                    if !original.contains(&path) {
                        return Some(path.port_name);
                    }
                }
            }
            sleep(Duration::from_millis(1000));
        }
    } else {
        out.println("> Couldn't access serial ports");
    }

    None
}
