use regex::{Regex, RegexSet};
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[macro_export]
macro_rules! error {
    ($expression:expr) => {
        eprintln!("[Error] {}", $expression);
    };
}

pub struct Preferences {
    pub color_enabled: bool,
}

fn parse(s: &str) {
    // Rust Regex can be tested here: https://rustexp.lpil.uk/

    // # command
    let user_input = Regex::new(r"^# ").unwrap();

    // ================
    let divider = Regex::new(r"(?m)^\s*(-|=|#)+\s*$").unwrap();

    //[ ===== Headline ====== ]
    let headline = Regex::new(r"^\[ =+ ?.* ?=+ \]").unwrap();

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

    fn printc(input: &str, color: Color, bold: bool) -> io::Result<()> {
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
            printc(s, Color::White, true)
        } else if divider.is_match(s) {
            printc(s, Color::Blue, false)
        } else if headline.is_match(s) {
            printc(s, Color::Yellow, true)
        } else if note.is_match(s) {
            printc(s, Color::Cyan, false)
        } else if error.is_match(s) {
            printc(s, Color::Red, false)
        } else if option.is_match(s) {
            printc(s, Color::Green, false)
        } else if default.is_match(s) {
            printc(s, Color::Green, true)
        } else if command.is_match(s) {
            printc(s, Color::Yellow, false)
        } else {
            printc(s, Color::White, false)
        }
    };

    match res {
        Ok(_) => {}
        Err(e) => error!(e),
    }
}

pub fn print(s: &str, p: &Preferences) {
    if p.color_enabled {
        parse(&s);
    } else {
        print!("{}", s);
    }
}

pub fn plug_in(pref: &Preferences) {
    print("> Plug in your device\r\n", pref);
}

pub fn no_access(pref: &Preferences) {
    print("> Couldn't access serial ports\r\n", pref);
}

pub fn connected(pref: &Preferences) {
    print("> Connected \\o/\r\n", pref);
}

pub fn logo() {
    let c_bytes = include_bytes!("visual/chicken.txt");
    let mut logo_str = String::from_utf8_lossy(c_bytes).to_string();
    println!("{}", logo_str);
}

pub fn version(pref: &Preferences) {
    let version = format!(" {} Version {} ", "Huhnitor", env!("CARGO_PKG_VERSION"));
    let headline = format!("[ {:=^76} ]\r\n", version);
    print(&headline, pref);
}

pub fn divider(pref: &Preferences) {
    let divider = format!("[ {:=^76} ]\r\n", '=');
    print(&divider, pref);
}

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn ports(ports: &std::vec::Vec<serialport::SerialPortInfo>, pref: &Preferences) {
    if ports.len() == 0 {
        print("> No serial devices found :(\r\n", pref);
    } else {
        print("Your available serial ports are:\r\n", pref);

        for (id, port) in ports.iter().enumerate() {
            let port = format!("[{}] {}\r\n", id, port.port_name);
            print(&port, pref);
        }
    }
}

pub fn no_ports(pref: &Preferences) {
    print("> No serial port found :(", pref);
    print("Make sure the USB connection works and necessary drivers are installed:\r\nhttps://github.com/SpacehuhnTech/Huhnitor#drivers", pref);
}
