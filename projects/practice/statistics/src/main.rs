use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [i32; n]
    }
    let mean = statistics::mean(&v);
    let median = statistics::median(&v);
    let modes = statistics::modes(&v);
    println!("mean={}, median={}, modes={:?}", mean, median, modes);
}
