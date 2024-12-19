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
        let numbers: Vec<i32> = input
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

        let mut total = i32::pow(2, (n - 1).try_into().unwrap());
        //println!("total: {}", total);

        if k > total {
            println!("{:?}", -1);
            continue;
        }

        let mut rem: i32 = k;
        let mut first: usize = 0;
        let mut last: usize = (n - 1) as usize;
        let mut vec: Vec<i32> = vec![0; n.try_into().unwrap()]; // Vector of size n, initialized to 0
        for num in 1..n {
            total = total / 2;
            if rem > total {
                vec[last] = num;
                last -= 1;
                rem -= total;
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
