use num::integer;
use std::ops::{Add, Mul, Sub};

fn main() {
    let sin1 = Frac::new(Interval::new(7761199951101802512, 7761199951101802513));
    let cos1 = Frac::new(Interval::new(4983409179392355912, 4983409179392355913));
    let sin2 = sin1 * cos1 + cos1 * sin1;
    let cos2 = cos1 * cos1 - sin1 * sin1;
    println!("{:?}", sin2);
    println!("{:?}", cos2);
    let sin3 = sin1 * cos2 + cos1 * sin2;
    let cos3 = cos1 * cos2 - sin1 * sin2;
    let sin4 = sin2 * cos2 + cos2 * sin2;
    let cos4 = cos2 * cos2 - sin2 * sin2;
    println!("1: {:x}", sin1.to_u32());
    println!("2: {:x}", sin2.to_u32());
    println!("3: {:x}", sin3.to_u32());
    println!("4: {:x}", sin4.to_u32());
    let sin8 = sin4 * cos4 + cos4 * sin4;
    let cos8 = cos4 * cos4 - sin4 * sin4;
    println!("8: {:x}", sin8.to_u32());
    let sin16 = sin8 * cos8 + cos8 * sin8;
    let cos16 = cos8 * cos8 - sin8 * sin8;
    println!("16: {:x}", sin16.to_u32());
    let sin32 = sin16 * cos16 + cos16 * sin16;
    let cos32 = cos16 * cos16 - sin16 * sin16;
    println!("32: {:x}", sin32.to_u32());
    let sin64 = sin32 * cos32 + cos32 * sin32;
    let cos64 = cos32 * cos32 - sin32 * sin32;
    println!("64: {:x}", sin64.to_u32());

    let mut sin = sin1;
    let mut cos = cos1;
    for i in 2..=64{
        let tmp = sin;
        sin = sin1*cos+cos1*sin;
        cos = cos1*cos-sin1*tmp;
        println!("{}: {:x}", i, sin.to_u32());
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
    num: Interval,//denom=2^63
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
