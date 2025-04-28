use std::io::{self, BufRead};

/*
Logic will be for this Coin Piles problem (https://cses.fi/problemset/task/1754)

2m + n = a
m + 2n = b

so for given a and b there should be valid m and n (which are whole numbers i.e. 1...)
n = (2b-a) / 3
m = (a-n) / 2
*/
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each test case
    for _ in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let mut parts = line.split_whitespace();
            let a: i64 = parts.next().unwrap().parse().unwrap();
            let b: i64 = parts.next().unwrap().parse().unwrap();

            let n = (2 * b - a) / 3;
            let m = (a - n) / 2;

            // check if they are valid whole numbers 0...n
            if m >= 0 && n >= 0 && 2 * m + n == a && m + 2 * n == b {
                println!("YES");
            } else {
                println!("NO",);
            }
        }
    }
}
