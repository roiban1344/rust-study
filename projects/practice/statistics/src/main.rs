use num::{integer, rational::Ratio};
use proconio::input;
use std::cmp::Ordering;
use std::collections::HashMap;

fn mean(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &e in v {
        sum += e;
    }
    let len = v.len() as i32;
    let floor = Ratio::from_integer(integer::div_floor(sum, len));
    let ceil = Ratio::from_integer(integer::div_ceil(sum, len));
    let mean = Ratio::new(sum, len);
    match (mean - floor).cmp(&(ceil - mean)) {
        Ordering::Greater => ceil.to_integer(),
        Ordering::Less => floor.to_integer(),
        Ordering::Equal => floor.to_integer(),
    }
}

fn median(v: &Vec<i32>) -> i32 {
    let mut w = v.clone();
    w.sort();
    let len = w.len();
    match integer::div_mod_floor(len, 2) {
        (div, 1) => (w[div] + w[div + 1]) / 2,
        (div, _) => w[div],
    }
}

fn modes(v: &Vec<i32>) -> Vec<i32> {
    let mut entries = HashMap::<i32, i32>::new();
    for e in v {
        let count = entries.entry(*e).or_insert(0);
        *count += 1;
    }
    let mut max_entry = 0;
    for &value in entries.values() {
        if max_entry < value {
            max_entry = value;
        }
    }
    let mut modes = Vec::<i32>::new();
    for (key, val) in entries {
        if val == max_entry{
            modes.push(key);
        }
    }
    modes
}

fn main() {
    input! {
        n: usize,
        v: [i32; n]
    }
    let mean = mean(&v);
    let median = median(&v);
    let modes = modes(&v);
    println!("mean={}, median={}, modes={:?}", mean, median, modes);
}
