fn main() {
    let n = 10;
    let mut s = Vec::<Vec<i32>>::new();
    for i in 1..=n {
        s.push(vec![i]);
    }
    let mut count = 0;
    while s.len() > 0 {
        let partition = s.pop().expect("empty stack");
        let &last = partition.last().expect("empty partition");
        let sum: i32 = partition.iter().sum();
        if sum == n {
            println!("{:?}", partition);
            count += 1;
        } else {
            let res = n - sum;
            for i in 1..=res.min(last) {
                let mut p = partition.clone();
                p.push(i);
                s.push(p);
            }
        }
    }
    println!("p({}) = {}", n, count);
}
