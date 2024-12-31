use std::io;

fn main() {
    let mut input = String::new();

    // Read all input from standard input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input for T"); // Number of test cases

    let mut test_cases = Vec::new(); // Store all test cases

    for _ in 0..t {
        // Read N
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let n: usize = input.trim().parse().expect("Invalid input for N");

        // Read the array
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let array: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input for array element"))
            .collect();

        // Ensure the length of the array matches N
        if array.len() != n {
            panic!("The number of elements in the array does not match N");
        }

        // Store the test case
        test_cases.push((n, array));
    }

    // Output the parsed test cases (for verification)
    //println!("Parsed Test Cases:");
    for (i, (n, array)) in test_cases.iter().enumerate() {
        //println!("Test Case {}: N = {}, Array = {:?}", i + 1, n, array);

        let mut found: bool = false;

        let mut l = 0;
        let mut r = (n - 1);
        let mut min: i32 = 1;
        let mut max: i32 = *n as i32;
        let mut found: bool = false;
        while l < r {
            if array[l] == min {
                l += 1;
                min += 1;
                continue;
            }
            if array[l] == max {
                l += 1;
                max -= 1;
                continue;
            }
            if array[r] == min {
                r -= 1;
                min += 1;
                continue;
            }
            if array[r] == max {
                r -= 1;
                max -= 1;
                continue;
            }

            println!("{} {}", l + 1, r + 1);
            found = true;
            break;
        }

        if !found {
            println!("-1");
        }
    }
}
