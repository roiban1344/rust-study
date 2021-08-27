use num::integer;
use std::ops::{Add, Mul, Sub};

fn main() {
    let sin1 = Frac::new(Interval::new(7761199951101802512, 7761199951101802513));
    let cos1 = Frac::new(Interval::new(4983409179392355912, 4983409179392355913));
    let sin2 = sin1 * cos1 + cos1 * sin1;
    let cos2 = cos1 * cos1 - sin1 * sin1;
    println!("{:?}", sin2);
    println!("{:?}", cos2);
    println!("1: {:x}", sin1.to_u32());
    println!("2: {:x}", sin2.to_u32());

    for i in 2..=64 {
        let sin = sin(&i);
        println!("{}: {:x}", i, sin.to_u32());
    }
}

fn sin(&i: &i32) -> Frac {
    if i == 1 {
        Frac::new(Interval::new(7761199951101802512, 7761199951101802513))
    } else {
        match i % 2 == 0 {
            true => {
                let j = i / 2;
                let tmp = sin(&j) * cos(&j);
                tmp + tmp
            },
            false => {
                let j = i / 2;
                sin(&j) * cos(&(j + 1)) + cos(&j) * sin(&(j + 1))
            },
        }
    }
}

fn cos(&i: &i32) -> Frac {
    if i == 1 {
        Frac::new(Interval::new(4983409179392355912, 4983409179392355913))
    } else {
        match i % 2 == 0 {
            true => {
                let j = i / 2;
                cos(&j) * cos(&j) - sin(&j) * sin(&j)
            },
            false => {
                let j = i / 2;
                cos(&j) * cos(&(j + 1)) - sin(&j) * sin(&(j + 1))
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Interval {
    min: i128,
    max: i128,
}

impl Interval {
    fn new(min: i128, max: i128) -> Interval {
        if min < 0 && 0 < max {
            panic!("Cannot contain 0.");
        } else if min > max {
            panic!("min = {} < max = {} is not satisfied.", min, max);
        }
        Interval { min, max }
    }

    fn is_positive(&self) -> bool {
        self.min > 0
    }

    fn sum(&self, other: &Interval) -> Interval {
        Interval {
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
    fn sub(&self, other: &Interval) -> Interval {
        Interval {
            min: self.min - other.max,
            max: self.max - other.min,
        }
    }
    fn prod(&self, other: &Interval) -> Interval {
        match (self.is_positive(), other.is_positive()) {
            (true, true) => Interval::new(self.min * other.min, self.max * other.max),
            (true, false) => Interval::new(self.max * other.min, self.min * other.max),
            (false, true) => Interval::new(self.min * other.max, self.max * other.min),
            (false, false) => Interval::new(self.max * other.max, self.min * other.min),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Frac {
    num: Interval, //denom=2^63
}

impl Frac {
    fn new(num: Interval) -> Frac {
        Frac { num }
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
    fn add(self, other: Self) -> Frac {
        Frac::new(self.num.sum(&other.num))
    }
}

impl Sub for Frac {
    type Output = Self;
    fn sub(self, other: Self) -> Frac {
        Frac::new(self.num.sub(&other.num))
    }
}

impl Mul for Frac {
    type Output = Self;
    fn mul(self, other: Self) -> Frac {
        let num_prod = self.num.prod(&other.num);
        let floor = integer::div_floor(num_prod.min, 1 << 63);
        let ceil = integer::div_ceil(num_prod.max, 1 << 63);
        let interval = Interval::new(floor, ceil);
        Frac::new(interval)
    }
}
