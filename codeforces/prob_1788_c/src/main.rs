use std::io;

fn main() {
    let mut input = String::new();

    // Read the entire input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the number of test cases
    let t: usize = input.trim().parse().expect("Expected a number");

    // Read T lines and process each test case
    for _ in 0..t {
        input.clear(); // Clear the input buffer for the next line
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let n: i32 = input.trim().parse().expect("Expected a number");

        // Add your processing logic here
        //println!("Processing test case with N = {}", n);

        if n % 2 == 0 {
            println!("no");
            continue;
        }

        println!("yes");

        let m: i32 = n / 2;
        //println!("m: {}", m);

        // pair the first m wih the last m
        // e.g. if n = 7
        // (1 12), (2 13), (3 14)
        // note the sums are 13, 15 and 17 respectively
        // In the second loop we will add 4 number pairs which sum to each side of the above
        // 12, 14, 16 and 18
        for i in 0..m {
            let f: i32 = i + 1;
            let s: i32 = 2 * n - m + i + 1;
            println!("{} {}", f, s);
        }

        // for remaining pair the next m+1 with the following m+1
        // e.g if n = 7
        for i in 0..m + 1 {
            let f: i32 = m + i + 1;
            let s: i32 = 2 * m + i + 2;
            println!("{} {}", f, s);
        }
    }
}
