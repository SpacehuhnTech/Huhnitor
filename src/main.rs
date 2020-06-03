use futures::stream::StreamExt;
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
    let settings = tokio_serial::SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(10),
    };

    let args: Vec<String> = env::args().collect();

    let no_color = args.iter().any(|arg| arg == "--no-color");
    let pref = output::Preferences{ color_enabled: !no_color };

    output::logo(&pref);
    output::version(&pref);

    let tty_path = if args.iter().any(|arg| arg == "-s") {
        port::manual(&pref)
    } else {
        port::auto(&pref)
    };   

    if let Some(inner_tty_path) = tty_path {
        #[allow(unused_mut)] // Ignore warning from windows compilers
        if let Ok(mut port) = tokio_serial::Serial::from_path(inner_tty_path, &settings) {

            #[cfg(unix)]
            port.set_exclusive(false)
                .expect("Unable to set serial port exclusive to false");

            let mut port = BufReader::new(port);

            let (sender, mut reciever) = tokio::sync::mpsc::unbounded_channel();
            tokio::spawn(input::receiver(sender));

            output::connected(&pref);
            output::divider(&pref);

            let mut buf = Vec::new();
            loop {
                tokio::select! {
                    len = port.read_until(b'\n', &mut buf) => match len {
                        Ok(0) => { // EOF
                            break;
                        },
                        Ok(_) => {
                            let input = String::from_utf8_lossy(&buf).to_string();
                            output::print(&input, &pref);
                            buf = Vec::new();
                        },
                        Err(e) => {
                            error!(e);
                            break;
                        }
                    },

                    Some(text_to_send) = reciever.next() => {
                        if port.write(text_to_send.as_bytes()).await.is_err() {
                            error!("Couldn't send message");
                        }
                    }
                }
            }
        } else { // Port handler
            error!("Couldn't open serial port!");
        }
    } else {
        output::no_ports(&pref);
    }
}
