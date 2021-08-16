fn main() {
    let mut i = 0;
    let j = 10;
    let x = loop {
        i += 1;
        if i >= j {
            break i;
        }
    };
    assert_eq!(x, j);

    let a = [1, 2, 3, 4, 5];
    for x in a {
        println!("{}", x);
    }

    for x in a.iter() {
        println!("{}", x);
    }
}
