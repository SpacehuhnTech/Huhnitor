use serialport::prelude::*;
use std::env;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[macro_use]
mod input;
mod output;
mod port;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    output::print_logo();

    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(input::receiver(sender));

    let tty_path = if args.iter().any(|arg| arg == "--no-auto") {
        port::manual(&mut receiver).await
    } else {
        port::auto(&mut receiver).await
    };

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
        if let Ok(mut port) = tokio_serial::Serial::from_path(&inner_tty_path, &settings) {

            #[cfg(unix)]
            port.set_exclusive(false)
                .expect("Unable to set serial port exclusive to false");

            let mut port = BufReader::new(port);

            output::print_connected(&inner_tty_path);

            let mut buf = Vec::new();
            loop {
                tokio::select! {
                    len = port.read_until(b'\n', &mut buf) => match len {
                        Ok(0) => { // EOF
                            break;
                        },
                        Ok(_) => {
                            output::print_input(&buf);
                            buf = Vec::new();
                        },
                        Err(e) => {
                            error!(e);
                            break;
                        }
                    },

                    Some(text_to_send) = receiver.recv() => {
                        if port.write(text_to_send.as_bytes()).await.is_err() {
                            error!("Couldn't send message");
                        }
                    }
                }
            }
        } else {
            // Port creation handler
            error!("Couldn't create port object!");
        }
    } else {
        // Path handler
        error!("No valid serial port found!");
    }
}
