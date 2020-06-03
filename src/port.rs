use serialport::available_ports;
use std::thread::sleep;
use std::time::Duration;

use crate::input;
use crate::output;

pub fn manual(pref: &output::Preferences) -> Option<String> {
    let available = available_ports().ok()?;

    output::ports(&available, pref);

    let port = input::read_line();

    Some(port)
}

pub fn auto(pref: &output::Preferences) -> Option<String> {
    if let Ok(original) = available_ports() {
        output::plug_in(pref);

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
        output::no_access(pref);
    }

    None
}
