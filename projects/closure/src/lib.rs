use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calcuration: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calcuration: T) -> Cacher<T> {
        Cacher {
            calcuration,
            values: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calcuration)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
