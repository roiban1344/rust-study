fn main() {
    let mut x = 1729;
    for _ in 0..10 {
        x = transform(x);
        println!("{}", x);
    }
}

fn transform(x: i32) -> i32 {
    let mut chars: Vec<char> = x.to_string().chars().collect();
    chars.sort();
    let max = chars
        .iter()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .expect("error");
    let min = chars
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .expect("error");
    max - min
}
