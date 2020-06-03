use regex::Regex;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[macro_export]
macro_rules! error {
    ($expression:expr) => {
        eprintln!("[Error] {}", $expression);
    };
}

fn printc(s: &str) {
    // # command
    let user_input = Regex::new(r"^# ").unwrap();

    // ================
    let divider = Regex::new(r"(?m)^\s*(-|=|#)+\s*$").unwrap();

    //[ ===== Headline ====== ]
    let headline = Regex::new(r"^\[ =+ .* =+ \]").unwrap();

    // > Finished job
    let note = Regex::new(r"^> \w+").unwrap();

    // ERROR: something went wrong :(
    let error = Regex::new(r"^(ERROR)|(WARNING): ").unwrap();

    // -arg value
    let option = Regex::new(r"^ {0,4}-?\S+.*: +\w+.*").unwrap();

    // [default=something]
    let default = Regex::new(r"^\[.*\]").unwrap();

    // command [-arg <value>] [-flag]
    let command = Regex::new(r"(?m)^\S+( \[?-\S*( <\S*>)?\]?)*\s*$").unwrap();

    fn print(input: &str, color: Color, bold: bool) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        stdout.set_color(
            ColorSpec::new()
                .set_fg(Some(color))
                .set_bg(Some(Color::Black))
                .set_bold(bold),
        )?;

        write!(&mut stdout, "{}", input)?;

        stdout.reset()
    }

    let res = {
        if user_input.is_match(s) {
            print(s, Color::White, true)
        } else if divider.is_match(s) {
            print(s, Color::Blue, false)
        } else if headline.is_match(s) {
            print(s, Color::Yellow, true)
        } else if note.is_match(s) {
            print(s, Color::Cyan, false)
        } else if error.is_match(s) {
            print(s, Color::Red, false)
        } else if option.is_match(s) {
            print(s, Color::Green, false)
        } else if default.is_match(s) {
            print(s, Color::Green, true)
        } else if command.is_match(s) {
            print(s, Color::Yellow, false)
        } else {
            print(s, Color::White, false)
        }
    };

    match res {
        Ok(_) => {}
        Err(e) => error!(e),
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

pub fn print_input(input: &Vec<u8>, color: bool) {
    let input_str = String::from_utf8_lossy(input);

    if color {
        printc(&input_str);
    } else {
        print!("{}", input_str);
    }
}

pub fn clear() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn print_no_serial_port() {
    error!("No serial port found :(");
    println!("Make sure the USB connection works and necessary drivers are installed:");
    println!("https://github.com/SpacehuhnTech/Huhnitor#drivers");
}
