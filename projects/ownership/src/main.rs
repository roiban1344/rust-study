fn main() {
    let mut x = 0;
    let a = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    for e in a {
        chmax(&mut x, e);
        println!("{}", x);
    }
}

fn chmax(x: &mut i32, y: i32) {
    if *x < y {
        *x = y;
    }
}
