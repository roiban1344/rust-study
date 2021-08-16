fn main() {
    let x = 127;
    let is_even = if x % 2 == 0 { true } else { false };
    if is_even {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }
}
