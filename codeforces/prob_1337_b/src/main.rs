use std::io;

fn main() {
    // Reading the number of test cases
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Expected a number");

    // Reading each test case
    for _ in 0..t {
        let mut case = String::new();
        io::stdin()
            .read_line(&mut case)
            .expect("Failed to read line");

        // Splitting the input line and parsing it into h, n, m
        let nums: Vec<i32> = case
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Expected a number"))
            .collect();

        let mut h = nums[0];
        let mut n = nums[1];
        let mut m = nums[2];

        // Output the parsed values for demonstration
        //println!("h: {}, n: {}, m: {}", h, n, m);
        let mut hzero = false;
        if h > 20 {
            while n > 0 {
                //println!("n is currently: {}", n);

                // Decrement n in each iteration
                h = h / 2 + 10;
                if h <= 0 {
                    hzero = true;
                    break;
                }
                n -= 1;
            }

            //println!("after n loop --> h: {h}");
            if hzero {
                println!("YES");
                continue;
            }
        }

        while m > 0 {
            h -= 10;
            m -= 1;
        }

        //println!("h: {h}");
        if h <= 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
