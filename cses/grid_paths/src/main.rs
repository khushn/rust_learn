use std::io::{self, Write};

const GRID_SIZE: usize = 7;

const NUM_STEPS: u32 = 48;

fn main() {
    print!("Enter input: ");
    io::stdout().flush().unwrap(); // Ensures the prompt is printed before input

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Remove any trailing newline characters
    let input = input.trim_end();

    println!("You entered: {}", input);

    

    // Create a 7x7 matrix initialized with zeros
    let mut matrix: [[u32; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    matrix[GRID_SIZE-1][0] = 1;
    let num_paths = find_path(0, 0, GRID_SIZE-1, 0, NUM_STEPS, &mut matrix);
    println!("{}", num_paths);
}

fn find_path(
    xb: usize,
    yb: usize,
    xe: usize,
    ye: usize,
    nsteps: u32,
    matrix: &mut [[u32; GRID_SIZE]; GRID_SIZE],
) -> u32 {
    // println!("xb: {}, yb: {} --> xe: {}, ye: {}, nsteps: {}", xb, yb, xe, ye, nsteps);
    if xb == xe && yb == ye {
        if nsteps == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if nsteps == 0 {
        return 0;
    }

    let mut npaths = 0;

    // find target's top neighbor
    let xt = xe as i32 - 1;
    if xt >= 0 && matrix[xt as usize][ye] == 0 {
        matrix[xt as usize][ye] = 1;
        npaths += find_path(xb, yb, xt as usize, ye, nsteps - 1, matrix);
        matrix[xt as usize][ye] = 0;
    }

    // find target's right neighbor
    let yt = ye as i32 + 1;
    if yt < GRID_SIZE as i32 && matrix[xe][yt as usize] == 0 {
        matrix[xe][yt as usize] = 1;
        npaths += find_path(xb, yb, xe, yt as usize, nsteps - 1, matrix);
        matrix[xe][yt as usize] = 0;
    }

    // find target's left neighbor
    let yt = ye as i32 - 1;
    if yt >= 0 && matrix[xe][yt as usize] == 0 {
        matrix[xe][yt as usize] = 1;
        npaths += find_path(xb, yb, xe, yt as usize, nsteps - 1, matrix);
        matrix[xe][yt as usize] = 0;
    }

    // find target's down neighbor
    let xt = xe as i32 + 1;
    if xt < GRID_SIZE as i32 && matrix[xt as usize][ye] == 0 {
        matrix[xt as usize][ye] = 1;
        npaths += find_path(xb, yb, xt as usize, ye, nsteps - 1, matrix);
        matrix[xt as usize][ye] = 0;
    }

    // println!("npaths: {}", npaths);
    return npaths;
}
