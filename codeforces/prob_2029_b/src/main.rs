use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read the value of n for the current test case
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Read the second line containing n numbers
        let nums: Vec<String> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();

        // Read the third line containing n-1 numbers
        let other_nums: Vec<String> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();

        // Insert your processing code here
        println!("Processing test case with n = {}", n);
        println!("nums = {:?}", nums);
        println!("other_nums = {:?}", other_nums);

        // Example processing code (you can replace or add to this)
        for num in &nums {
            println!("Processing num: {}", num);
        }
        for other_num in &other_nums {
            println!("Processing other_num: {}", other_num);
        }
    }
}
