use std::io::{self, BufRead};

/*
Logic:
Loop 1 to n , remove trailing zeros of each no. and add to count.
Preserve only the last digit after multiplying. If multiplication results in a new trailing zero,
account for it and remove it
*/
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut num_trailing_zeros: u32 = 0;
    let mut mult: u64 = 1;
    for num in 2..=n {
        //println!("processing num: {:?}", num);
        let mut i = num;
        while i % 5 == 0 {
            i = i / 5;
            num_trailing_zeros += 1;
        }
    }

    println!("{:?}", num_trailing_zeros);
}
