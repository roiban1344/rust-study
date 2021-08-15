fn main() {
    let x = {
        let x = 3;
        println!("x = {}", x);
        x
    };
    let y = {
        let y = 1;
        println!("y = {}", y);
        y;
    };

    assert_eq!(x, 3);
    assert_eq!(y, ());

    let z = add(8, 13);
    assert_eq!(z, 21);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
