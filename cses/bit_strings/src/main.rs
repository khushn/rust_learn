use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let DIV: u32 = 1000000007;

    let mut ans: u32 = 1;
    for i in 0..n {
        ans = (ans * 2) % DIV;
    }

    println!("{:?}", ans);
}
