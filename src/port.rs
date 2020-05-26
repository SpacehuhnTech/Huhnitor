use serialport::available_ports;
use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;

pub fn manual() -> Option<String> {
    let available = available_ports().ok()?;
    print!("Your available ports are: ");
    for port in available.iter() {
        print!("{} ", port.port_name);
    }
    println!();

    let mut port = String::new();
    stdin().read_line(&mut port).ok()?;
    port = port.trim().to_string();

    Some(port)
}

pub fn auto() -> Option<String> {
    if let Ok(original) = available_ports() {
        println!("Plug in your device...");
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
        println!("Couldn't access serial ports!");
    }
    None
}
