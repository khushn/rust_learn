use std::io;

fn main() {
    // Read the first line: number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Read the second line: space-separated integers
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read numbers");
    let nums: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Your processing code goes here
    let expected_sum: u64 = (n * (n+1) / 2).try_into().unwrap();
    let mut sum: u64 = 0;
    for n in nums {
    	sum += n as u64;
    }

    println!("{:?}", expected_sum - sum);
}
