use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

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

    let throw = "投";
    let catch = "掴";
    //let try = "試";
}

fn read_user_name_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
