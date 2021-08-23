use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_name = "hello.txt";
    let f = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem crating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}
