use md5;

fn main() {
    let digest = md5::compute(b"Rust");
    println!("{:x}", digest);
}
