use std::io;

fn main() {
    let mut input = String::new();

    // Read the entire input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the number of test cases
    let t: usize = input
        .trim()
        .parse()
        .expect("Invalid input for number of test cases");

    // Create a vector to store the test cases
    let mut test_cases = Vec::new();

    // Read each test case
    for _ in 0..t {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse the pair of integers (n, k)
        let parts: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input for n or k"))
            .collect();

        if parts.len() == 2 {
            test_cases.push((parts[0], parts[1])); // Push the tuple (n, k)
        } else {
            panic!("Invalid test case format");
        }
    }

    // Process each test case
    for (n, k) in test_cases {
        //println!("Processing test case: n = {}, k = {}", n, k);

        let (lv, num) = star_gaze(1, n, k);
        println!("{}", lv);
    }
}


/*
Optimized recursive function to invoke only the left most branch of the 
tree and compute the rest.
*/
fn star_gaze(b: u32, e: u32, k: u32) -> (u64, u64) {
    if b > e {
        return (0, 0);
    }
    //println!("b: {}, e: {}", b, e);

    let l = e - b + 1;

    if l < k {
        return (0, 0);
    }

    if l == 1 {
        return (b as u64, 1);
    }

    let m = (b + e) / 2;
    if l % 2 == 0 {
        let (v1, n1) = star_gaze(b, m, k);

        // instead of invoking for 2nd half we derive using the values of left half
        // this will solve TLE issue
        // This works as they are symmetrical we just need to add n1*m to the result
        //let (v2, n2) = star_gaze(m + 1, e, k);
        (v1 + v1 + n1 * m as u64, n1 * 2)
    } else {
        let (v1, n1) = star_gaze(b, m - 1, k);

        // instead of invoking for 2nd half we derive using the values of left half
        // this will solve TLE issue
        // This works as they are symmetrical we just need to add n1*(m+1) to the result
        // let (v2, n2) = star_gaze(m + 1, e, k);
        (m as u64 + v1 + v1 + n1 * m as u64, n1 * 2 + 1)
    }
}
