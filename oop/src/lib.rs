use std::collections::HashSet;

#[derive(Debug)]
pub struct AveragedCollection {
    //list: Vec<i32>,
    list: HashSet<i32>,
    average: f64,
}

impl AveragedCollection {
    // Constructor (new) to create an instance
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            // list: Vec::new(),
            list: HashSet::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        //self.list.push(value);
        self.list.insert(value);
        self.update_average();
    }

    pub fn remove(&mut self, x: i32) -> Option<i32> {
        // let result = self.list.pop();
        let result = self.list.remove(&x);
        /*
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
        */
        if result {
            self.update_average();
            Some(x)
        } else {
            None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}