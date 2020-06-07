use regex::RegexSet;
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
    static ref REGSET: RegexSet = RegexSet::new(&[
        r"^# ",                                 // # command
        r"(?m)^\s*(-|=|#)+\s*$",                // ================
        r"^\[ =+ ?.* ?=+ \]",                   // [ ===== Headline ====== ]
        r"^> \w+",                              // > Finished job
        r"^(ERROR)|(WARNING): ",                // ERROR: something went wrong :(
        r"^ {0,4}-?\S+.*: +\w+.*",              // -arg value
        r"^\[.*\]",                             // [default=something]
        r"(?m)^\S+( \[?-\S*( <\S*>)?\]?)*\s*$", // command [-arg <value>] [-flag]
    ]).unwrap();

    static ref COLORSET: Vec<(Color, bool)> = vec![
        (Color::White, true),   // # command
        (Color::Blue, false),   // ================
        (Color::Yellow, true),  // [ ===== Headline ====== ]
        (Color::Cyan, false),   // > Finished job
        (Color::Red, false),    // ERROR: something went wrong :(
        (Color::Green, false),  // -arg value
        (Color::Green, true),   // [default=something]
        (Color::Yellow, false), // command [-arg <value>] [-flag]
    ];
}

fn parse(s: &str) {
    let matches: Vec<_> = REGSET.matches(s).into_iter().collect();

    let (color, bold) = if !matches.is_empty() {
        COLORSET[matches[0]]
    } else {
        (Color::White, false)
    };

    fn print_color(input: &str, color: Color, bold: bool) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        stdout.set_color(
            ColorSpec::new()
                .set_fg(Some(color))
                //.set_bg(Some(Color::Black))
                .set_bold(bold),
        )?;

        write!(&mut stdout, "{}", input)?;
        stdout.reset()
    }

    if let Err(e) = print_color(s, color, bold) {
        error!(e);
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
    let logo_str = String::from_utf8_lossy(c_bytes).to_string();
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
