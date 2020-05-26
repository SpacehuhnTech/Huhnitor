use futures::stream::StreamExt;
use serialport::prelude::*;
use std::env;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod input;
mod port;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Display chicken
    let c_bytes = include_bytes!("visual/chicken.txt");
    println!("{}", String::from_utf8_lossy(c_bytes));

    let tty_path = if args.iter().any(|arg| arg == "-s") {
        port::manual()
    } else {
        port::auto()
    };

    // Define settings (support for changing planned)
    let settings = tokio_serial::SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(10),
    };

    if let Some(inner_tty_path) = tty_path {
        #[allow(unused_mut)] // Ignore warning from windows compilers
        let mut port = tokio_serial::Serial::from_path(inner_tty_path, &settings).unwrap();

        #[cfg(unix)]
        port.set_exclusive(false)
            .expect("Unable to set serial port exclusive to false");

        let mut port = BufReader::new(port);

        let (sender, mut reciever) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(input::receiver(sender));
        let mut buf = Vec::new();
        println!("Connected!");

        loop {
            tokio::select! {
                len = port.read_until(b'\n', &mut buf) => match len {
                    Ok(0) => break, // EOF
                    Ok(_) => { print!("{}", String::from_utf8_lossy(&buf)); buf = Vec::new(); },
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
