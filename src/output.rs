pub fn print_logo() {
    let c_bytes = include_bytes!("visual/chicken.txt");
    println!("{}", String::from_utf8_lossy(c_bytes));
}

pub fn print_ports() {
    print!("Your available ports are: ");
}

pub fn print_port(port_name: &String) {
    println!("{}", port_name);
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

pub fn error(e: &str) {
    eprintln!("[Error] {}", e);
}
