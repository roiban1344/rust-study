use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let mut terminal = BTreeMap::<i32, &BTreeSet<i32>>::new();
    for mut n in 1_000..10_000 {
        let mut met = Vec::<i32>::new();
        while !met.contains(&n) {
            met.push(n);
            n = transform(n);
        }
        if(terminal.contains_key(&n)){
            terminal[&n].insert(met[0]);
        }else{
            let mut set = BTreeSet::<i32>::new();
            set.insert(met[0]);
            terminal.insert(n, &(set.clone()));
        }
        println!("{} {} {}", met[0], met.len(), n);
    }
}

fn transform(x: i32) -> i32 {
    let mut a = Vec::<i32>::new();
    let mut x = x;
    while x > 0 {
        let q = x / 10;
        a.push(x - q * 10);
        x = q;
    }
    a.sort();
    let mut y = 0;
    for e in a.iter().rev() {
        y = y * 10 + e;
    }
    let mut z = 0;
    for e in a.iter() {
        z = z * 10 + e;
    }
    y - z
}
