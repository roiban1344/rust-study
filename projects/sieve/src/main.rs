fn main() {
    let n = 100;
    let mut is_prime = vec![true; n + 1];
    let mut primes = vec![];
    for d in 2..=n {
        if is_prime[d] {
            primes.push(d);
            for i in (2 * d..=n).step_by(d) {
                is_prime[i] = false;
            }
        }
    }
    for p in primes {
        println!("{}", p);
    }
}
