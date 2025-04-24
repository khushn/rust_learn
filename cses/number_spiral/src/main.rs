use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each test case
    for _ in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let mut parts = line.split_whitespace();
            let y: u64 = parts.next().unwrap().parse().unwrap();
            let x: u64 = parts.next().unwrap().parse().unwrap();

            // Your processing and output code goes here
            let (min, max, row_max) = min_max(y, x);

            let contained_sq = (max - 1) * (max - 1);

            /*
            println!(
                "min: {}, max: {}, row_max: {}, contained_sq: {}",
                min, max, row_max, contained_sq
            );
            */

            let mut ans: u64 = 0;
            if row_max {
                if max % 2 == 0 {
                    ans = max * max - (min - 1);
                } else {
                    ans = contained_sq + min;
                }
            } else {
                if max % 2 == 0 {
                    ans = contained_sq + min;
                } else {
                    ans = max * max - (min - 1);
                }
            }

            println!("{:?}", ans);
        }
    }
}

fn min_max<T: Ord>(x: T, y: T) -> (T, T, bool) {
    if x <= y {
        (x, y, false)
    } else {
        (y, x, true)
    }
}
