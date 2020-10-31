use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use std::collections::VecDeque;
use std::time::{Instant, Duration};

use crate::error;

pub fn receiver(sender: UnboundedSender<String>) {
    let mut exitspam: VecDeque<Instant> = VecDeque::with_capacity(3);

    let mut rl = rustyline::Editor::<()>::new();
    rl.bind_sequence(rustyline::KeyPress::Up, rustyline::Cmd::LineUpOrPreviousHistory(1));
    rl.bind_sequence(rustyline::KeyPress::Down, rustyline::Cmd::LineDownOrNextHistory(1));

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                if sender.send(line.clone()).is_err() {
                    error!("Couldn't report input to main thread!");
                }

                if line.trim().to_uppercase() == "EXIT" {
                    break;
                }
            },
            Err(rustyline::error::ReadlineError::Interrupted) => {
                sender.send("stop\n".to_string()).expect("Couldn't stop!");

                if exitspam.len() == 3 {
                    if let Some(time) = exitspam.pop_back() {
                        if Instant::now() - time <= Duration::new(3, 0) {
                            sender.send("EXIT".to_string()).expect("Couldn't exit!");
                            break;
                        } else {
                            exitspam.push_front(Instant::now());
                        }
                    }
                } else {
                    exitspam.push_front(Instant::now());
                }
            }
            Err(e) => error!(e) 
            
        }
    }
}

pub async fn read_line(receiver: &mut UnboundedReceiver<String>) -> Option<String> {
    Some(receiver.recv().await?.trim().to_string())
}
