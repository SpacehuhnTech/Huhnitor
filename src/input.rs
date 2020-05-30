use crate::output;
use std::io;

pub async fn receiver(sender: tokio::sync::mpsc::UnboundedSender<String>) {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if input.trim() == "clear" {
                output::clear();
            } else {
                sender.send(input).unwrap();
            }
        }
    }
}

pub fn read_line() -> String {
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Failed to read line");

    val.trim().to_string()
}
