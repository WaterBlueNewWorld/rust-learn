fn main() {
    let mut a = 0;
    loop {
        if a == 15 {
            break;
        }
        println!("{}", a);
        a = a + 1;
    }
}