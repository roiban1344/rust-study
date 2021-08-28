use num::integer;
use std::ops::{Add, Mul, Sub};

const RANGE_MAX: usize = 8192;

macro_rules! frac_interval {
    [$min:expr, $max:expr] => [
        Frac::new(Interval::new($min, $max))
    ]
}

fn main() {
    let mut sin = vec![Option::<Frac>::None; RANGE_MAX + 1];
    let mut cos = vec![Option::<Frac>::None; RANGE_MAX + 1];
    sin[1] = Some(frac_interval![7761199951101802512, 7761199951101802513]);
    cos[1] = Some(frac_interval![4983409179392355912, 4983409179392355913]);

    let print_data = |i: i32, frac: &Frac| -> () {
        println!(
            "{0},{1:#x},{1},{2},{3},{4}",
            i,
            frac.to_u32(),
            frac.num.min,
            frac.num.max,
            frac.num.max - frac.num.min
        )
    };

    print_data(1, &sin[1].unwrap());

    for i in 2..=RANGE_MAX {
        let s1 = sin[i / 2].unwrap();
        let c1 = cos[i / 2].unwrap();
        let s2 = sin[(i + 1) / 2].unwrap();
        let c2 = cos[(i + 1) / 2].unwrap();
        sin[i] = Some(s1 * c2 + c1 * s2);
        cos[i] = Some(c1 * c2 - s1 * s2);
        print_data(i as i32, &sin[i].unwrap());
    }
}

#[derive(Copy, Clone, Debug)]
struct Interval {
    min: i128,
    max: i128,
}

// enum Sign{
//     Pos,
//     Neg,
// }

impl Interval {
    fn new(min: i128, max: i128) -> Interval {
        if min < 0 && 0 < max {
            panic!("Cannot contain 0.");
        } else if min > max {
            panic!("min = {} < max = {} is not satisfied.", min, max);
        }
        Interval { min, max }
    }

    fn sign(&self) -> i32 {
        if self.min > 0 && self.max > 0 {
            //Sign::Pos
            1
        } else if self.min < 0 && self.max < 0 {
            //Sign::Neg
            -1
        } else {
            panic!("Contains 0 {:?}", self);
        }
    }
}

impl Add for Interval {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Interval {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
}

impl Sub for Interval {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Interval {
            min: self.min - other.max,
            max: self.max - other.min,
        }
    }
}

impl Mul for Interval {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let a = self.min;
        let b = self.max;
        let c = other.min;
        let d = other.max;
        let (min, max) = match (self.sign(), other.sign()) {
            (1, 1) => (a * c, b * d),
            (1, -1) => (b * c, a * d),
            (-1, 1) => (a * d, b * c),
            (-1, -1) => (b * d, a * c),
            _ => panic!("unreachable"),
        };
        Interval::new(min, max)
    }
}

#[derive(Debug, Copy, Clone)]
struct Frac {
    num: Interval, //denom=2^63
}

impl Frac {
    fn new(num: Interval) -> Self {
        Self { num }
    }
    fn to_u32(&self) -> u32 {
        let min = integer::div_floor(self.num.min.abs(), 1 << 31);
        let max = integer::div_floor(self.num.max.abs(), 1 << 31);
        if min == max {
            min as u32
        } else {
            println!("{:?}", self.num);
            panic!("min={}, max={}", min, max);
        }
    }
}

impl Add for Frac {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Frac::new(self.num + other.num)
    }
}

impl Sub for Frac {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Frac::new(self.num - other.num)
    }
}

impl Mul for Frac {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let num_prod = self.num * other.num;
        let floor = integer::div_floor(num_prod.min, 1 << 63);
        let ceil = integer::div_ceil(num_prod.max, 1 << 63);
        frac_interval![floor, ceil]
    }
}
