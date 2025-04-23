use std::io;

fn main() {
    // Read input string from stdin
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Remove the trailing newline (if any)
    let input = input.trim();

    // Iterate over characters
    let mut i: i32 = 0;
    let mut prev: char = ' ';
    let mut max: i32 = 0;
    let mut reps: i32 = 0;
    for ch in input.chars() {
        if i == 0 {
            reps = 1;
            max = 1;
            i += 1;
            prev = ch;
            continue;
        }
        if prev == ch {
            reps += 1;
        } else {
            if reps > max {
                max = reps;
            }
            reps = 1;
        }

        // Do something with each character
        //println!("{}", ch); // Replace this with your processing code

        i += 1;
        prev = ch;
    }

    if reps > max {
        max = reps;
    }

    println!("{:?}", max);
}
