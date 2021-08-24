fn main() {
    let list = [3, 2, 4, 5, 1];
    println!("The largest numer is {}", largest(&list).unwrap());
}

fn largest(list: &[i32]) -> Option<i32> {
    if list.len() == 0 {
        return None;
    }
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}
