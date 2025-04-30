use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let num_moves = count_moves(n as u32);
    println!("{}", num_moves);
    make_moves(n as u32, 1, 2, 3);
}

fn count_moves(n: u32) -> u32 {
    if n == 1 {
        return n;
    }

    let nminone = count_moves(n - 1);
    return (nminone * 2 + 1);
}

fn make_moves(n: u32, source: u32, using: u32, target: u32) {
    if n == 1 {
        println!("{} {}", source, target);
        return;
    }

    make_moves(n - 1, source, target, using);
    println!("{} {}", source, target);
    make_moves(n - 1, using, source, target);
}
