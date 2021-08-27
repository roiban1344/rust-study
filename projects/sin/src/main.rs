use std::cmp::PartialEq;
use std::cmp::Ordering;
use num::integer;//::{div_ceil, div_floor};

fn main() {
    let sin1 = Frac::new(Interval::new(7761199951101802512, 7761199951101802513));
    let cos1 = Frac::new(Interval::new(4983409179392355912, 4983409179392355913));
    let sin2 = {
        let tmp = sin1.prod(&cos1);
        tmp.sum(&tmp)
    };
    let cos2 = cos1.prod(&cos1).sub(&sin1.prod(&sin1));
    println!("{:?}", sin2);
    println!("{:?}", cos2);
    println!("{:x}", integer::div_floor(sin1.num.min, 1<<31));
    println!("{:x}", integer::div_floor(sin2.num.min, 1<<31));
    let sin3 = sin1.prod(&cos2).sum(&cos1.prod(&sin2));
    let cos3 = cos1.prod(&cos2).sub(&sin1.prod(&sin2));
    println!("{:x}", integer::div_floor(sin3.num.min, 1<<31));
    let sin4 = sin2.prod(&cos2).sum(&cos2.prod(&sin2));
    let cos4 = cos2.prod(&cos2).sub(&sin2.prod(&sin2));
    println!("{:x}", integer::div_floor(-sin4.num.min, 1<<31));
}

// #[derive(PartialEq)]
#[derive(Copy, Clone, Debug)]
struct Interval{
    min: i128,
    max: i128,
}

// impl PartialOrd for Interval{
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if  self.max < other.min {
//             Some(Ordering::Less)
//         }else if self.min > other.max {
//             Some(Ordering::Greater)
//         } else {
//             Some(Ordering::Equal)
//         }
//     }
// }

impl Interval{
    fn new(min:i128, max:i128) -> Interval{
        if min < 0 && 0 < max{
            panic!("Cannot contain 0.");
        } else if min > max {
            panic!("min = {} < max = {} is not satisfied.", min, max);
        }
        Interval{
            min,
            max
        }
    }

    fn is_positive(&self) -> bool{
        self.min > 0
    }
    // fn is_negative(&self) -> bool{
    //     self.max < 0
    // }

    fn sum(&self, other: &Interval) -> Interval{
        Interval{
            min: self.min + other.min,
            max: self.max + other.max,
        }
    }
    fn sub(&self, other:&Interval) -> Interval{
        Interval{
            min: self.min - other.max,
            max: self.max - other.min,
        }
    }
    fn prod(&self, other: &Interval) -> Interval{
        match (self.is_positive(), other.is_positive()) {
            (true, true) => Interval::new(
                self.min * other.min,
                self.max * other.max,
            ),
            (true, false) => Interval::new(
                self.max * other.min,
                self.min * other.max,
            ),
            (false, true) => Interval::new(
                self.min * other.max,
                self.max * other.min,
            ),
            (false, false) => Interval::new(
                self.max * other.max,
                self.min * other.min
            ),
        }
    }
}

#[derive(Debug)]
struct Frac{
    num: Interval
    //denom: 2^63
}

const DENOM:i128 = 1<<63;

impl Frac{
    fn new(num: Interval) -> Frac{
        Frac{
            num
        }
    }

    fn sum(&self, other:&Self) -> Frac{
        Frac::new(self.num.sum(&other.num))
    }
    fn sub(&self, other:&Self)->Frac{
        Frac::new(self.num.sub(&other.num))
    }
    fn prod(&self, other:&Self)->Frac{
        let num_prod = self.num.prod(&other.num);
        let floor = integer::div_floor(num_prod.min, DENOM);
        let ceil = integer::div_ceil(num_prod.max, DENOM);
        let interval = Interval::new(floor, ceil);
        Frac::new(interval)
    }
}

