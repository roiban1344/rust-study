fn main() {
    let n: u32 = 100;
    let mut is_prime = vec![true; (n + 1) as usize];
    let mut primes = vec![];
    for d in 2..=n {
        if is_prime[d as usize] {
            primes.push(d);
            for i in (2 * d..=n).step_by(d as usize) {
                is_prime[i as usize] = false;
            }
        }
    }
    for p in primes {
        println!("{}", p);
    }
}
