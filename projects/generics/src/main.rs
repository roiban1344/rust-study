use std::cmp::PartialOrd;

fn main() {
    let list = [3, 2, 4, 5, 1];
    println!("1: The largest number is {}", largest1(&list).unwrap());
    println!("2: The largest number is {}", largest2(&list).unwrap());
    println!("3: The largest number is {}", largest3(&list).unwrap());
}

fn largest1<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
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

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> Option<T> {
    if list.len() == 0 {
        return None;
    }
    let mut largest = list[0].clone();
    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }
    Some(largest)
}

fn largest3<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.len() == 0 {
        return None;
    }
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    Some(&largest)
}
