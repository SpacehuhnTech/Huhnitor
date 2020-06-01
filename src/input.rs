use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use std::io;

pub async fn receiver(sender: UnboundedSender<String>) {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            sender.send(input).unwrap();
        }
    }
}

pub async fn read_line(receiver: &mut UnboundedReceiver<String>) -> Option<String> {
    Some(receiver.recv().await?.trim().to_string())
}
