fn main() {
    let mut text = String::new();
    let mut hola = String::new();
    hola.push_str("HOLA SASJKSGFJAS");
    println!("Ingresa una palabra: {}", hola);
    let _intext = std::io::stdin().read_line(&mut text).unwrap();
    let _state = match text.trim() {
        "XD" => println!("LOOOOLOLOLOLOLXDXDDX"),
        "HGDFJD" => println!("SAS"),
        _ => println!("NO"),
    };
}