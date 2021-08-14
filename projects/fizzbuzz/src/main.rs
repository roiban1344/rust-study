fn main() {
    for i in 1..=100 {
        let line = match (i % 3, i % 5) {
            (0, 0) => String::from("fizzbuzz"),
            (0, _) => String::from("fizz"),
            (_, 0) => String::from("buzz"),
            (_, _) => i.to_string(),
        };
        println!("{}", line);
    }
}
