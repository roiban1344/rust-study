use rectangles::Rectangle;

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 10,
    };
    let square = Rectangle::square(50);
    println!("{:#?}", rect1);
    println!("area: {}", rect1.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect2.can_hold(&square));
    println!("{}", square.can_hold(&rect1));
}
