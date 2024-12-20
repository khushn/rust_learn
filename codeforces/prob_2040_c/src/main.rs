use std::io;

fn main() {
    let mut input = String::new();

    // Read the first line: the number of test cases
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().expect("Invalid number of test cases");

    // Initialize a vector to store the test cases
    let mut test_cases = Vec::new();

    // Read the test cases
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let numbers: Vec<i64> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();

        // Push the tuple (n, k) into the vector
        test_cases.push((numbers[0], numbers[1]));
    }

    // Process each test case
    for (n, k) in test_cases {
        // Placeholder for your logic
        //println!("Processing: n = {}, k = {}", n, k);

        // let mut total: i64 = i32::pow(2, (n - 1) as u32) as i64;
        let mut total: i64 = n - 1;
        //println!("total: {}", total);

        let klog2 = (k as f64).log2();

        if klog2 > total as f64 {
            println!("{:?}", -1);
            continue;
        }

        let mut rem: i64 = k;
        let mut first: usize = 0;
        let mut last: usize = (n - 1) as usize;
        let mut vec: Vec<i64> = vec![0; n.try_into().unwrap()]; // Vector of size n, initialized to 0
        for num in 1..n {
            total -= 1;
            let remlog2 = (rem as f64).log2();
            //println!("remlog2: {:?}, total: {}", remlog2, total);
            if remlog2 > total as f64 {
                vec[last] = num;
                last -= 1;
                rem -= i64::pow(2, total as u32) as i64;
            } else {
                vec[first] = num;
                first += 1;
            }
        }
        vec[first] = n;

        for i in 0..n {
            print!("{} ", vec[i as usize]);
        }
        println!("");
    }
}
