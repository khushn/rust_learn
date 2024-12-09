use std::io::{self, BufRead};

fn main() {
    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for test_case in 1..=t {
        // Read n and m
        let line = lines.next().unwrap().unwrap();
        let mut dimensions = line.split_whitespace();
        let n: usize = dimensions.next().unwrap().parse().unwrap();
        let m: usize = dimensions.next().unwrap().parse().unwrap();

        // Read the maze directions
        let mut maze: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            let row = lines.next().unwrap().unwrap();
            maze.push(row.chars().collect());
        }

        // Print test case for debugging (optional)
        //println!("Test Case {}: n = {}, m = {}", test_case, n, m);
        for row in &maze {
            // println!("{:?}", row);
        }

        let mut data: Vec<Vec<usize>> = vec![vec![0; m]; n];

        // use dfs to set all the outbound paths
        for i in 0..n {
            for j in 0..m {
                if data[i][j] == 0 {
                    //println!("called");
                    dfs(&maze, &mut data, i, j);
                }
            }
        }

        // now just look for '?' in maze and ensure all neighbors are not 1
        for i in 0..n {
            for j in 0..m {
                if maze[i][j] == '?' {
                    let mut outway: usize = 0;

                    if i == 0 {
                        outway += 1;
                    }

                    if j == 0 {
                        outway += 1;
                    }

                    if i == n - 1 {
                        outway += 1;
                    }

                    if j > 0 && data[i][j - 1] == 1 {
                        outway += 1;
                    }

                    if j < m - 1 && data[i][j + 1] == 1 {
                        outway += 1;
                    }

                    if j == m - 1 {
                        outway += 1;
                    }

                    if i > 0 && data[i - 1][j] == 1 {
                        outway += 1;
                    }

                    if i < n - 1 && data[i + 1][j] == 1 {
                        outway += 1;
                    }

                    if outway == 4 {
                        data[i][j] = 1;
                    }

                    //println!("processing '?'s i:{:?}, j:{:?}, outway: {:?}", i, j, outway);
                }
            }
        }

        // now we
        for row in &data {
            //println!("{:?}", row);
        }

        let mut ret: usize = 0;
        for i in 0..n {
            for j in 0..m {
                if data[i][j] == 0 {
                    ret += 1;
                }
            }
        }

        println!("{:?}", ret);
    }
}

fn dfs(maze: &Vec<Vec<char>>, data: &mut Vec<Vec<usize>>, i: usize, j: usize) {
    //println!("dfs called with i:{:?}, j:{:?}", i, j);

    // Boundary check to ensure (i, j) is within the maze dimensions
    let N: usize = maze.len();
    let M: usize = maze[0].len();
    if i < 0 || j < 0 || i >= N || j >= M {
        return;
    }

    // Example: Check if this cell has already been processed
    if data[i][j] != 0 {
        //println!("{:?}", data[i][j]);
        return;
    }

    // Logic to process the current cell based on `maze[i][j]`
    // For example, move in the direction specified by `maze[i][j]`

    let mut oneset: bool = false;

    match maze[i][j] {
        'U' => {
            if i == 0 || (i > 0 && data[i - 1][j] == 1) {
                data[i][j] = 1;
                oneset = true;
            }
        }
        'D' => {
            if i == N - 1 || (i < N - 1 && data[i + 1][j] == 1) {
                data[i][j] = 1;
                oneset = true;
            }
        }
        'L' => {
            if j == 0 || (j > 0 && data[i][j - 1] == 1) {
                data[i][j] = 1;
                oneset = true;
            }
        }
        'R' => {
            if j == M - 1 || (j < M - 1 && data[i][j + 1] == 1) {
                data[i][j] = 1;
                oneset = true;
            }
        }
        '?' => {
            // ignore
        }
        _ => {}
    }

    if oneset {
        if i + 1 < N && maze[i + 1][j] != '?' && data[i + 1][j] == 0 {
            dfs(maze, data, i + 1, j); // down
        }

        if i >= 1 && maze[i - 1][j] != '?' && data[i - 1][j] == 0 {
            dfs(maze, data, i - 1, j); // up
        }

        if j + 1 < M && maze[i][j + 1] != '?' && data[i][j + 1] == 0 {
            dfs(maze, data, i, j + 1); // right
        }

        if j >= 1 && maze[i][j - 1] != '?' && data[i][j - 1] == 0 {
            dfs(maze, data, i, j - 1); // left
        }
    }
}
