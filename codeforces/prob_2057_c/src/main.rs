use std::io;

fn main() {
    // Read the number of test cases
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input
        .trim()
        .parse()
        .expect("Failed to parse number of test cases");

    // Initialize a vector to hold the test cases
    let mut test_cases = Vec::new();

    // Read each test case
    for _ in 0..t {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let nums: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();

        if nums.len() == 2 {
            test_cases.push((nums[0], nums[1]));
        } else {
            panic!("Each test case must contain exactly two numbers");
        }
    }

    // Process each test case
    for (l, r) in test_cases {
        // Replace this with your desired processing logic
        println!("Processing range: {} to {}", l, r);
        let l_bin = to_binary(l);
        let r_bin = to_binary(r);
        println!("The binary representation of {} is {}", l, l_bin);
        println!("The binary representation of {} is {}", r, r_bin);
    }
}

fn to_binary(n: u32) -> String {
    format!("{:b}", n)
}
