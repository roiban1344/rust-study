use my_box::MyBox;

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //same as
    hello(&(*m)[..]);
}
