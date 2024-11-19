/*
Trying to solve this problem
https://puzzles.nigelcoldwell.co.uk/fourteen.htm
*/
fn main() {
    const SIZE: usize = 52;
    const numcards: usize = SIZE / 2 + 1;

    // Create a 2D arr of size 52x53 initialized with zeros
    let arr: [[f32; numcards]; numcards] = [[0.; numcards]; numcards];

    // Access and modify an element, for example, the element at row 0, column 0
    let mut arr = arr; // mutable arr to modify values
                       //arr[0][0] = 42;

    // Print the element at row 0, column 0
    println!("Element at [0][0]: {}", arr[0][0]);

    // end state after all the cards have expired
    let mut p: usize = 0;
    let mut n: usize = 0;

    let mut prob_left: [[f32; numcards]; numcards] = [[0.; numcards]; numcards];
    let mut prob_up: [[f32; numcards]; numcards] = [[0.; numcards]; numcards];
    prob_left[0][0] = 0.0;
    prob_up[0][0] = 0.0;

    // initialization of expected move values
    for i in 1..numcards {
        arr[0][i] = (i - 1) as f32;
        //arr[i][0] = (i - 1) as f32 * -1.0;
    }

    let mut cur_val_arr: [[f32; numcards]; numcards] = [[0.; numcards]; numcards];
    for r in 0..numcards {
        for c in 0..numcards {
            let mut cur_val: f32 = (numcards - 1 - r) as f32 - (numcards - 1 - c) as f32;
            cur_val_arr[r][c] = cur_val;
        }
    }

    let mut dec_arr: Vec<Vec<char>> = Vec::new();

    for r in 0..numcards {
        let mut dec_row: Vec<char> = Vec::new();
        for c in 0..numcards {
            let mut neg: usize = c;
            let mut pos: usize = r;

            // probability of moving left (getting a negative card)
            let mut pleft: f32 = neg as f32 / (pos as f32 + neg as f32);

            // prob of moving up (getting a +ve card)
            let mut pup: f32 = pos as f32 / (pos as f32 + neg as f32);

            if pos + neg > 0 {
                prob_left[r][c] = pleft;

                prob_up[r][c] = pup;
            }

            // expected value of moving left

            let mut expected_val: f32 = 0.0;
            if r > 0 && c > 0 {
                expected_val = pleft * arr[r][c - 1] + pup * arr[r - 1][c];
                arr[r][c] = cur_val_arr[r][c];
                if expected_val > cur_val_arr[r][c] {
                    arr[r][c] = expected_val;
                }
            }

            //println!("{:?}, {:?} = {:?}", r, c, arr[r][c]);

            let mut move_or_not: char = 'y';
            if expected_val < cur_val_arr[r][c] {
                move_or_not = 'x';
            }

            dec_row.push(move_or_not);
        }
        dec_arr.push(dec_row);
    }

    dec_arr[0][0] = 'x';

    println!("-- prob_left --");
    for row in &prob_left {
        println!("{:?}", row);
    }

    println!("-- prob_up --");
    for row in &prob_up {
        println!("{:?}", row);
    }

    println!("-- cur_val_arr --");
    for row in &cur_val_arr {
        println!("{:?}", row);
    }

    println!("-- expected val arr --");
    for row in &arr {
        println!("{:?}", row);
    }

    println!("-- dec_arr --");
    for row in &dec_arr {
        println!("{:?}", row);
    }
}
