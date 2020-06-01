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
    println!("Plug in your device, enter a path, or enter an index...");
}

pub fn print_connected(port: &str) {
    println!("Connected to {} \\o/", port);
    println!("=================================================================================")
}

pub fn print_input(input: &Vec<u8>) {
    let input_str = String::from_utf8_lossy(input);
    print!("{}", input_str);
}

#[macro_export]
macro_rules! error {
    ($expression:expr) => {
        eprintln!("[Error] {}", $expression);
    };
}
