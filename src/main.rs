use serialport::prelude::*;
use std::env;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[macro_use]
mod input;
mod output;
mod port;

struct Arguments {
    help: bool,
    driver: bool,
    auto: bool,
}

fn parse_args() -> Arguments {
    let mut args = Arguments {
        help: false,
        driver: false,
        auto: true,
    };

    let words: Vec<String> = env::args().collect();

    for word in words[1..].iter() {
        match word.as_ref() {
            "--help" | "-h" => args.help = true,
            "--driver" | "-d" => args.driver = true,
            "--no-auto" | "-na" => args.auto = false,
            _ => println!("Wrong parameter..."),
        }
    }

    args
}

async fn monitor(auto: bool) {
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
    tokio::spawn(input::receiver(sender));

    let settings = tokio_serial::SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(10),
    };

    let mut tty_path = None;

    if auto {
        tty_path = port::auto(&mut receiver).await;
    } else {
        tty_path = port::manual(&mut receiver).await;
    }

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

#[tokio::main]
async fn main() {
    output::print_logo();

    let args = parse_args();

    if args.help {
        output::help();
    } else if args.driver {
        output::driver();
    } else {
        monitor(args.auto).await;
    }

    output::goodbye();
}
