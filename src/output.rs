pub fn print_logo() {
    let c_bytes = include_bytes!("visual/chicken.txt");
    println!("{}", String::from_utf8_lossy(c_bytes));
}

pub fn print_ports(ports: &std::vec::Vec<serialport::SerialPortInfo>) {
    if ports.is_empty() {
        println!("No serial devices found :(");
        println!("Consider using the --help or --driver argument if you're having trouble.");
    } else {
        println!("Your available serial ports are: ");

        for (id, port) in ports.iter().enumerate() {
            println!("[{}] {}", id, port.port_name);
        }
    }
}

pub fn print_plug_in() {
    println!("Connect your Deauther or enter the port number/name");
}

pub fn print_connected(port: &str) {
    println!("=================================================================================");
    println!("Connected to {} \\o/", port);
    println!("=================================================================================");
}

pub fn print_input(input: &Vec<u8>) {
    let input_str = String::from_utf8_lossy(input);
    print!("{}", input_str);
}

pub fn help() {
    println!("Args ============================================================================");
    println!("--help    => print the help screen");
    println!("--no-auto => manually select serial port");
    println!("--driver  => open link to probable driver in your deafult browser");
    println!("=================================================================================");
}

pub fn driver() {
    if webbrowser::open("https://github.com/spacehuhntech/huhnitor#drivers").is_err() {
        println!("Couldn't open URL :(");
        println!("Visit \"https://github.com/spacehuhntech/huhnitor#drivers\"");
    }
}

#[macro_export]
macro_rules! error {
    ($expression:expr) => {
        eprintln!("[Error] {}", $expression);
    };
}
