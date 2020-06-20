use regex::RegexSet;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[macro_export]
macro_rules! error {
    ($expression:expr) => {
        eprintln!("[Error] {}", $expression);
    };
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

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(" ");
}

pub struct Preferences {
    pub color_enabled: bool,
}

impl Preferences {
    pub fn print(&self, s: &str) {
        if self.color_enabled {
            parse(&s);
        } else {
            print!("{}", s);
        }
    }

    pub fn println(&self, s: &str) {
        self.print(s);
        println!();
    }

    pub fn logo(&self) {
        let c_bytes = include_bytes!("visual/chicken.txt");
        let logo_str = String::from_utf8_lossy(c_bytes).to_string();
        println!("{}", logo_str);
    }

    pub fn version(&self) {
        let version = format!(" Huhnitor Version {} ", env!("CARGO_PKG_VERSION"));
        let headline = format!("[ {:=^76} ]", version);
        self.println(&headline);
    }

    pub fn divider(&self) {
        let divider = format!("[ {:=^76} ]", '=');
        self.println(&divider);
    }

    pub fn ports(&self, ports: &[serialport::SerialPortInfo]) {
        if ports.is_empty() {
            self.hint();
        } else {
            self.println("Available serial ports:");
            for (id, port) in ports.iter().enumerate() {
                let port = format!("[{}] {}", id, port.port_name);
                self.println(&port);
            }
        }
    }

    pub fn hint(&self) {
        self.println("> No serial port found");
        self.println("Make sure the USB connection works and necessary drivers are installed:");
        self.println("https://github.com/SpacehuhnTech/Huhnitor#drivers");
    }

    pub fn connected(&self, port: &str) {
        let msg = format!("Connected to {} \\o/", port);

        self.println(&msg);
        self.divider();
    }

    pub fn driver(&self) {
        self.print("Opening \"https://github.com/spacehuhntech/huhnitor#drivers\"...");

        if webbrowser::open("https://github.com/spacehuhntech/huhnitor#drivers").is_err() {
            self.println("Couldn't open URL :(");
        } else {
            self.println("OK")
        }
    }

    pub fn goodbye(&self) {
        let bye = format!("[ {:=^76} ]", " Thanks for using Huhnitor ");
        self.println(&bye);
    }
}
