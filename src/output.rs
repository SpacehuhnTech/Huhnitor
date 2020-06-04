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

// Statically compile regex to avoid repetetive compiling
// Rust Regex can be tested here: https://rustexp.lpil.uk/
lazy_static::lazy_static! {
    // # command
    static ref USER_INPUT: Regex = Regex::new(r"^# ").unwrap();

    // ================
    static ref DIVIDER: Regex = Regex::new(r"(?m)^\s*(-|=|#)+\s*$").unwrap();

    //[ ===== Headline ====== ]
    static ref HEADLINE: Regex = Regex::new(r"^\[ =+ ?.* ?=+ \]").unwrap();

    // > Finished job
    static ref NOTE: Regex = Regex::new(r"^> \w+").unwrap();

    // ERROR: something went wrong :(
    static ref ERROR: Regex = Regex::new(r"^(ERROR)|(WARNING): ").unwrap();

    // -arg value
    static ref OPTION: Regex = Regex::new(r"^ {0,4}-?\S+.*: +\w+.*").unwrap();

    // [default=something]
    static ref DEFAULT: Regex = Regex::new(r"^\[.*\]").unwrap();

    // command [-arg <value>] [-flag]
    static ref COMMAND: Regex = Regex::new(r"(?m)^\S+( \[?-\S*( <\S*>)?\]?)*\s*$").unwrap();

    // set
    static ref REGSET: RegexSet = RegexSet::new(&[
        r"^# ",
        r"(?m)^\s*(-|=|#)+\s*$",
        r"^\[ =+ ?.* ?=+ \]",
        r"^> \w+",
        r"^(ERROR)|(WARNING): ",
        r"^ {0,4}-?\S+.*: +\w+.*",
        r"^\[.*\]",
        r"(?m)^\S+( \[?-\S*( <\S*>)?\]?)*\s*$",
    ]).unwrap();
}

fn parse(s: &str) {
    let matches: Vec<_> = REGSET.matches(s).into_iter().collect();

    let colors: Vec<(Color, bool)> = vec![
        (Color::White, true),
        (Color::Blue, false),
        (Color::Yellow, true),
        (Color::Cyan, false),
        (Color::Red, false),
        (Color::Green, false),
        (Color::Green, true),
        (Color::Yellow, false)
    ];

    let (color, bold) = if !matches.is_empty() {
        colors[matches[0]]
    } else {
        (Color::White, false)
    };

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(color))
            .set_bold(bold),
    ).unwrap();
    write!(&mut stdout, "{}", s);
    stdout.reset();
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
