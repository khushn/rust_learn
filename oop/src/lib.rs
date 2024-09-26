use std::collections::HashSet;

// trait draw
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

/*
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

*/

// The above doesn't work, hence commented (it uses the T type of the first val added in the Vec)
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        println!("In Screen run()");
        for component in self.components.iter() {
            component.draw();
        }
    }
}


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
