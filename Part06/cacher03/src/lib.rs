#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    hashmap: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            hashmap: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.hashmap.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.hashmap.insert(arg, v);
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
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }
}
