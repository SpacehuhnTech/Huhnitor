use std::fs::File;
use std::io::prelude::*;

use crate::error;

pub fn handle(command: String) -> String {
    let words = command.split(' ').collect::<Vec<&str>>();
    let len = words.len();
    if let Some(key) = words.get(1) {
        match key.to_uppercase().trim().as_ref() {
            "READ" => {
                if len > 2 {
                    let mut out = String::new();
                    if let Ok(mut file) = File::open(words[2].trim()) {
                        if file.read_to_string(&mut out).is_err() {
                            error!(format!("Coudldn't read file: '{}'", words[1]));
                        }
                    } else {
                        error!(format!("Couldn't open file: '{}'", words[2].trim()));
                    }

                    return out;
                } else {
                    println!("Insufficient arguments");
                    println!("Command format: read [filename]");
                }
            }

            _ => error!("Command not found"),
        }
    }

    String::new()
}
