fn main() {
    let sum = add(34.0, 67.0);
    show(sum);
}

fn add(a: f64, b: f64) -> f64 {
    a+b
}

fn show(s: f64){
    println!("{:?}", s);
}