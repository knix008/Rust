#![allow(dead_code)]
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    
    pub fn average(&self) -> f64 {
        self.average
    }
    
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = (total as f64) / (self.list.len() as f64);
    }
}

fn main() {
    let mut avc = AveragedCollection { list: vec![], average: 0.0 };
    avc.add(1);
    avc.add(2);
    avc.add(3);
    println!("The avc : {:?}.", avc);
    println!("The average : {:.32}.", avc.average());

    avc.remove();
    avc.remove();
    avc.remove();
    println!("The avc : {:?}.", avc);
}