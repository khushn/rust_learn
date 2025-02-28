mod front_of_house;

mod back_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // to use the enums
    let s = back_of_house::Appetizer::Soup;
    println!("enum val: {:?}", s);
}

// Below code was generated when we created this project using
// cargo new restaurant --lib

pub fn add(left: usize, right: usize) -> usize {
    left + right
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
    fn test_add_to_waitlist() {
        super::front_of_house::hosting::add_to_waitlist();
        assert_eq!(1, 1);
    }

    #[test]
    fn test_eat_at_restaurant() {
        eat_at_restaurant();
        assert_eq!(0, 0);
    }
}
