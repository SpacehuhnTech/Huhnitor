use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serialport::prelude::*;
use serialport::available_ports;
use futures::stream::StreamExt;

use std::time::Duration;
use std::thread::sleep;
use std::io::stdin;
use std::env;

mod input;

fn auto_serial() -> Option<String> {
    if let Ok(original) = available_ports() {
        println!("Plug in your device...");
        for _ in 0..30 {
            if let Ok(paths) = available_ports() {
                for path in paths {
                    if !original.contains(&path) {
                        return Some(path.port_name)
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

fn manual_serial() -> Option<String> {
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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Display chicken
    let c_bytes = include_bytes!("visual/chicken.txt");
    println!("{}", String::from_utf8_lossy(c_bytes));

    let tty_path = if args.iter().any(|arg| arg == "-s") {
        // Manual serial input
        manual_serial()
    } else {
        // Detect serial device when plugged in
        auto_serial()
    };

    // Define settings (support for changing planned)
    let settings = tokio_serial::SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(10)
    };

    if let Some(inner_tty_path) = tty_path {
        let mut port = tokio_serial::Serial::from_path(inner_tty_path, &settings).unwrap();

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        let mut port = BufReader::new(port);

        let (sender, mut reciever) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(input::input_receiver(sender));
        let mut buf = Vec::new();
        println!("Connected!");

        loop {
            tokio::select! {
                len = port.read_until(b'\n', &mut buf) => match len {
                    Ok(0) => break, // EOF
                    Ok(_) => println!("{}", String::from_utf8_lossy(&buf)),
                    Err(e) => { eprintln!("[ERR] {}", e); break; }
                },

                Some(text_to_send) = reciever.next() => {
                    if port.write(text_to_send.as_bytes()).await.is_err() {
                        println!("[SEND ERR]");
                    }   
                }   
            }   
        }
    } else {
        println!("No valid serial port found!");
    }
}