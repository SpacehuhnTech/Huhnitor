use std::io;

pub async fn receiver(sender: tokio::sync::mpsc::UnboundedSender<String>) {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            sender.send(input).unwrap();
        }
    }
}
