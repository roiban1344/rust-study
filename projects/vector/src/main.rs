fn main() {
    let mut v: Vec<i32> = Vec::new();
    let w = vec![1, 2, 3];
    v.push(5);
    v.push(6);

    let n = 6;

    let e = &v[n];
    println!("Get value with []: {}", e);

    match v.get(n){
        Some(e) => println!("Get value with .get(): {}", e),
        None => println!(""),
    }
}
