fn main() {
    let mut text = String::new();
    println!("Ingresa una palabra");
    let _intext = std::io::stdin().read_line(&mut text).unwrap();
    let _state = match text.trim() {
        "XD" => println!("LOOOOLOLOLOLOLXDXDDX"),
        "HGDFJD" => println!("SAS"),
        _ => println!("NO"),
    };
}