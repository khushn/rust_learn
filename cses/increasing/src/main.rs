use std::io;

fn main() {
    // Read the first line: number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Read the second line: space-separated integers
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read numbers");
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Your processing code goes here
    let mut prev: i32 = 0;
    let mut ans: u64 = 0;

    for (i, num) in nums.iter().enumerate() {
        // println!("Index: {}, Value: {}", i, n);
        // You can now use 'index' to access the element's position

        if i == 0 {
        	prev = *num;
        	continue
        }

        if *num < prev {
        	ans += (prev - *num) as u64;
        } else {
        	prev = *num;
        }
    }

    println!("{:?}", ans);

}
