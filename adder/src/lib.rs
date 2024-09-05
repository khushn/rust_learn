pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
        assert_eq!(4, 4);
    }

    #[test]
    fn another2() {
        // passed
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
