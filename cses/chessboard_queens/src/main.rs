use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut board: Vec<Vec<char>> = Vec::new();

    //println!("Enter the board lines. Press Ctrl+D (Linux/macOS) or Ctrl+Z then Enter (Windows) to end input.");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        let row: Vec<char> = line.chars().collect();
        board.push(row);
    }

    let rows = board.len();
    let cols = if rows > 0 { board[0].len() } else { 0 };

    //println!("Board size: {} rows x {} columns", rows, cols);
    //println!("Board contents:");
    /*
    for row in &board {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    */

    let mut placed: Vec<usize> = Vec::new();
    let num_poss = get_num_poss(rows, 0, &mut placed, &mut board);
    println!("{}", num_poss);
}

fn get_num_poss(n: usize, row: usize, placed: &mut Vec<usize>, board: &mut Vec<Vec<char>>) -> u64 {
    let mut num_poss: u64 = 0;
    for col in 0..n {
        if board[row][col] == '*' {
            continue;
        }

        let mut col_allowed = true;
        for r in 0..row {
            let c = placed[r];
            if c == col {
                col_allowed = false;
                break;
            }

            let vdiff = (row as i32 - r as i32).abs();
            let hdiff = (col as i32 - c as i32).abs();
            if vdiff == hdiff {
                col_allowed = false;
                break;
            }
        }

        if col_allowed {
            if row + 1 < n {
                placed.push(col);
                num_poss += get_num_poss(n, row + 1, placed, board);
                placed.remove(placed.len() - 1);
            } else {
                num_poss += 1;
            }
        }
    }

    return num_poss;
}
