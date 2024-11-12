use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read the value of n for the current test case
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Read the second line containing n numbers
        let nums: Vec<String> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();

        // Read the third line containing n-1 numbers
        let other_nums: Vec<String> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(String::from)
            .collect();

        // Insert your processing code here
        //println!("Processing test case with n = {}", n);
        //println!("nums = {:?}", nums);
        //println!("other_nums = {:?}", other_nums);

        // Example processing code (you can replace or add to this)
        let mut ones: i32 = 0;
        let mut zeros: i32 = 0;
        let mut printed_no: bool = false;
        for num in &nums {
            //println!("Processing num: {}", num);
            for c in num.chars() {
                if c == '1' {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
        }

        //println!("num ones: {}, num zeros: {}", ones, zeros);

        for other_num in &other_nums {
            //println!("Processing other_num: {}", other_num);
            for c in other_num.chars() {
                if zeros == 0 || ones == 0 {
                    println!("NO");
                    printed_no = true;
                    break;
                }

                if c == '1' {
                    zeros -= 1;
                } else {
                    ones -= 1;
                }
            }

            if printed_no {
                break;
            }
        }

        if printed_no {
            continue;
        }

        println!("YES");
    }
}
