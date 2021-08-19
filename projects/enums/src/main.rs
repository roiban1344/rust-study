enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let _four = IpAddr::V4;
    let _six = IpAddr::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let _home = IpAddr::V4(127,0,0,1);
    
    // let loopback = IpAddr {
        //     kind: IpAddrKind::V6,
        //     address: String::from("::1"),
        // };
    let _loopback = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();
}