use handler::handle;
use serialport::prelude::*;
use std::env;
use std::time::Duration;
use structopt::StructOpt;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[macro_use]
mod handler;
mod input;
mod output;
mod port;

async fn monitor(auto: bool, out: &output::Preferences) {
    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

    std::thread::spawn(|| input::receiver(sender));

    let settings = tokio_serial::SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(10),
    };

    let tty_path = if auto {
        port::auto(&mut receiver, out).await
    } else {
        port::manual(&mut receiver, out).await
    };

    if let Some(inner_tty_path) = tty_path {
        #[allow(unused_mut)] // Ignore warning from windows compilers
        if let Ok(mut port) = tokio_serial::Serial::from_path(&inner_tty_path, &settings) {
            #[cfg(unix)]
            port.set_exclusive(false)
                .expect("Unable to set serial port exclusive to false");

            let mut port = BufReader::new(port);

            out.connected(&inner_tty_path);

            //let mut stdout = std::io::stdout().into_raw_mode().unwrap();

            let mut buf = Vec::new();
            loop {
                tokio::select! {
                    len = port.read_until(b'\n', &mut buf) => match len {
                        Ok(0) => { // EOF
                            break;
                        },
                        Ok(_) => {
                            let input = String::from_utf8_lossy(&buf).to_string();
                            out.print(&input);
                            buf = Vec::new();
                        },
                        Err(e) => {
                            error!(e);
                            break;
                        }
                    },

                    Some(text) = receiver.recv() => {
                        if text.trim().to_uppercase() == "EXIT" {
                            break;
                        } else if text.trim().to_uppercase() == "CLEAR" {
                            output::clear();
                        } else if text.to_uppercase().starts_with("HUHN") {
                            if port.write(handle(text).as_bytes()).await.is_err() {
                                error!("Command failed");
                            }
                        } else if port.write(text.as_bytes()).await.is_err() {
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
        out.hint();
    }
}

#[derive(StructOpt)]
#[structopt(name = "Huhnitor", about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// Open driver page
    #[structopt(short, long)]
    driver: bool,

    /// Disable automatic port connection
    #[structopt(short = "a", long = "no-auto")]
    auto: bool,

    /// Disable colored output
    #[structopt(short = "c", long = "no-color")]
    color: bool,
}

#[tokio::main]
async fn main() {
    let args = Opt::from_args();

    let out = output::Preferences {
        color_enabled: !args.color,
    };

    out.logo();
    out.version();

    if args.driver {
        out.driver();
    } else {
        monitor(!args.auto, &out).await;
    }

    out.goodbye();
}
