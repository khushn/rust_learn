use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let t: usize = input
        .trim()
        .parse()
        .expect("Invalid input for number of test cases");
    let mut test_cases = Vec::new();

    for _ in 0..t {
        // Read n, m, k
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let values: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input for n, m, k"))
            .collect();
        let (n, m, k) = (values[0], values[1], values[2]);

        // Read binary string
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let binary_string = input.trim().to_string();

        // Store the test case data
        test_cases.push((n, m, k, binary_string));
    }

    for (i, (n, m, k, binary_string)) in test_cases.iter().enumerate() {
        /*
        println!(
            "Processing test case {}: n={}, m={}, k={}, binary_string={}",
            i + 1,
            n,
            m,
            k,
            binary_string
        );
        */

        let mut res: usize = 0;

        let mut vec: Vec<usize> = Vec::new();
        let mut prevc: char = 'x';
        let mut cur_zero_len: usize = 0;

        let chars: Vec<char> = binary_string.chars().collect();

        let mut i: usize = 0;
        while i < *n {
            let mut c: char = chars[i];
            if c == '0' {
                cur_zero_len += 1;
            } else {
                cur_zero_len = 0;
            }

            if cur_zero_len >= *m {
                res += 1;
                i += k;
                cur_zero_len = 0;
            } else {
                i += 1;
            }
        }

        /*
        // Add your processing code here
        for (i, c) in binary_string.chars().enumerate() {
            //println!("Character at index {}: {}", i, c);

            if c == '0' {
                cur_zero_len += 1;
            } else {
                if cur_zero_len >= *m {
                    vec.push(cur_zero_len);
                }
                cur_zero_len = 0;
            }

            if cur_zero_len == *m {}
        }

        if cur_zero_len >= *m {
            vec.push(cur_zero_len);
        }

        //println!("vec: {:?}", vec);

        for (i, g) in vec.into_iter().enumerate() {
            //println!("g: {:?}", g);
            let mut num = g - m + 1;
            let mut denom = k + m - 1;
            let mut t = num / denom;
            if num % denom != 0 {
                t += 1;
            }
            //println!("num: {:?}, denom: {:?}, t: {:?}", num, denom, t);
            res += t;
        }
        */

        println!("{:?}", res);
    }
}
