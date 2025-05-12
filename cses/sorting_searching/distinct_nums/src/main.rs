use std::io;
use std::collections::HashMap;


fn main() {
    // Read the number of elements
    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).expect("Failed to read line");
    let n: usize = first_line.trim().parse().expect("Expected an integer");

    // Read the list of integers
    let mut second_line = String::new();
    io::stdin().read_line(&mut second_line).expect("Failed to read line");

    let numbers: Vec<i32> = second_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Expected a number"))
        .collect();

    assert_eq!(numbers.len(), n, "Number of elements doesn't match declared size");

    // You can do your processing here
    //println!("You entered {} numbers: {:?}", n, numbers);
    let mut nums_map = HashMap::new();

    for n in numbers {
    	*nums_map.entry(n).or_insert(0) += 1;
    }

    //println!("nums_map: {:?}", nums_map);
    println!("{}", nums_map.len());
}
