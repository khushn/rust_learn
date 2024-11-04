use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        //println!("-------");
        // Read each line with `n` and `k`
        if let Some(Ok(line)) = lines.next() {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let (n, k) = (nums[0], nums[1]);

            // Insert your inner loop or logic here
            //println!("n: {}, k: {}", n, k); // Example output, replace with actual logic

            // boundary case solution
            if n == 1 && k == 1 {
                println!("1\n1");
                continue;
            }

            if k == n || k == 1 {
                println!("-1");
                continue;
            }

            // else we have a solution
            println!("3");

            let mut mid: u32 = k;
            let mut last: u32 = k + 1;
            if k % 2 != 0 {
                mid = k - 1;
                last = k + 2;
            }
            println!("{:?} {:?} {:?}", 1, mid, last);
        }
    }
}
