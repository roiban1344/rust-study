fn main() {
    let mut x = 1729;
    println!("{}", x);
    for _ in 0..10 {
        x = transform(x);
        println!("{}", x);
    }
}

fn transform(x: i32) -> i32 {
    let mut a = Vec::<i32>::new();
    let mut x = x;
    while x > 0 {
        let q = x / 10;
        a.push(x - q * 10);
        x = q;
    }
    a.sort();
    let mut y = 0;
    for e in a.iter().rev() {
        y = y * 10 + e;
    }
    let mut z = 0;
    for e in a.iter() {
        z = z * 10 + e;
    }
    y - z
}
