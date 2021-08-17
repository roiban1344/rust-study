fn main() {
    let n = 6;
    let mut s = Vec::<Vec<i32>>::new();
    for i in 1..=n {
        s.push(vec![i]);
    }
    let mut count = 0;
    loop {
        let partition = match s.pop() {
            Some(partition) => partition,
            None => break,
        };
        let &last = match partition.last() {
            Some(last) => last,
            None => break,
        };
        let sum: i32 = partition.iter().sum();
        if sum == n {
            println!("{:?}", partition);
            count += 1;
        } else {
            let res = n - sum;
            for i in 1..=res.min(last) {
                let mut partition = partition.clone();
                partition.push(i);
                s.push(partition);
            }
        }
    }
    println!("p({}) = {}", n, count);
}
