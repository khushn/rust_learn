use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each test case
    for _ in 0..t {
        // Read N and M
        let first_line = lines.next().unwrap().unwrap();
        let mut nm = first_line.split_whitespace();
        let n: usize = nm.next().unwrap().parse().unwrap();
        let m: usize = nm.next().unwrap().parse().unwrap();

        // Read the vector of N+M-1 characters
        let char_line = lines.next().unwrap().unwrap();
        let char_vec: Vec<char> = char_line.chars().collect();

        // Read the NxM grid
        let mut grid = Vec::new();
        for _ in 0..n {
            let grid_line = lines.next().unwrap().unwrap();
            let row: Vec<i64> = grid_line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            grid.push(row);
        }

        // Debug output (replace with your processing logic)
        //println!("N: {}, M: {}", n, m);
        //println!("Characters: {:?}", char_vec);
        //println!("Grid: {:?}", grid);

        let mut i: usize = 0;
        let mut j: usize = 0;
        for ch in char_line.chars() {
            //println!("{}", ch);
            if ch == 'D' {
                // sum the row should add up to 0
                let mut sum: i64 = 0;
                for c in 0..m {
                    if c == j {
                        continue;
                    }
                    sum += grid[i][c];
                }
                grid[i][j] = -sum;
                i += 1;
            } else {
                // ch == 'R'
                // sum of columns should add up to 0
                let mut sum: i64 = 0;
                for r in 0..n {
                    if r == i {
                        continue;
                    }
                    sum += grid[r][j];
                }
                grid[i][j] = -sum;
                j += 1;
            }
        }

        // For cell at bottom right
        let mut sum: i64 = 0;
        for r in 0..n - 1 {
            sum += grid[r][m - 1]
        }
        grid[n - 1][m - 1] = -sum;

        //println!("Final Grid: {:?}", grid);
        for i in 0..n {
            for j in 0..m {
                print!("{:?} ", grid[i][j]);
            }
            println!("");
        }
    }
}
