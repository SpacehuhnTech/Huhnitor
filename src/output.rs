pub fn print_logo() {
    let c_bytes = include_bytes!("visual/chicken.txt");
    println!("{}", String::from_utf8_lossy(c_bytes));
}

pub fn print_connected() {
    println!("Connected!");
}

pub fn print_input(input: &Vec<u8>) {
    let input_str = String::from_utf8_lossy(input);
    print!("{}", input_str);
}
