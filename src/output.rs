#[macro_export]
macro_rules! error {
    ($expression:expr) => {
        eprintln!("[Error] {}", $expression);
    };
}

pub fn print_logo() {
    let c_bytes = include_bytes!("visual/chicken.txt");
    println!("{}", String::from_utf8_lossy(c_bytes));
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

pub fn print_no_serial_port() {
    error!("No serial port found :(");
    println!("Make sure the USB connection works and necessary drivers are installed:");
    println!("https://github.com/SpacehuhnTech/Huhnitor#drivers");
}
