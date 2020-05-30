use colored::*;
use regex::Regex;

fn printc(s: &str) {
    let user_input = Regex::new(r"^# ").unwrap();
    let divider = Regex::new(r"(?m)^\s*(-|=|#)+\s*$").unwrap();
    let headline = Regex::new(r"^\[ =+ .* =+ \]").unwrap();
    let note = Regex::new(r"^> \w+").unwrap();
    let error = Regex::new(r"^(ERROR)|(WARNING): ").unwrap();
    let option = Regex::new(r"^ {0,4}-?\S+.*: +\w+.*").unwrap();
    let default = Regex::new(r"^\[.*\]").unwrap();
    let command = Regex::new(r"(?m)^\S+( \[?-\S*( <\S*>)?\]?)*\s*$").unwrap();

    if user_input.is_match(s) {
        print!("{}", s.white().bold());
    } else if divider.is_match(s) {
        print!("{}", s.blue());
    } else if headline.is_match(s) {
        print!("{}", s.yellow().bold());
    } else if note.is_match(s) {
        print!("{}", s.cyan());
    } else if error.is_match(s) {
        print!("{}", s.red());
    } else if option.is_match(s) {
        print!("{}", s.green());
    } else if default.is_match(s) {
        print!("{}", s.green().bold());
    } else if command.is_match(s) {
        print!("{}", s.yellow());
    } else {
        print!("{}", s.white());
    }
}

pub fn print_logo() {
    let c_bytes = include_bytes!("visual/chicken.txt");
    println!("{}", String::from_utf8_lossy(c_bytes));

    let version_str = format!(" {} Version {} ", "Huhnitor", env!("CARGO_PKG_VERSION"));
    let headline = format!("[ {:=^76} ]", version_str);
    println!("{}", headline);
}

pub fn print_ports(ports: &std::vec::Vec<serialport::SerialPortInfo>) {
    if ports.len() == 0 {
        println!("No serial devices found :(");
    } else {
        println!("Your available serial ports are: ");

        for (id, port) in ports.iter().enumerate() {
            println!("[{}] {}", id, port.port_name);
        }
    }
}

pub fn print_plug_in() {
    println!("Plug in your device...");
}

pub fn print_no_access() {
    println!("Couldn't access serial ports!");
}

pub fn print_connected() {
    println!("Connected!");
}

pub fn print_input(input: &Vec<u8>) {
    let input_str = String::from_utf8_lossy(input);
    print!("{}", input_str);
}
pub fn clear() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[macro_export]
macro_rules! error {
    ($expression:expr) => {
        eprintln!("[Error] {}", $expression);
    };
}
