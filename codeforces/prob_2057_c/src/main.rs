use std::io;

fn main() {
    // Read the number of test cases
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input
        .trim()
        .parse()
        .expect("Failed to parse number of test cases");

    // Initialize a vector to hold the test cases
    let mut test_cases = Vec::new();

    // Read each test case
    for _ in 0..t {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let nums: Vec<u32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Failed to parse number"))
            .collect();

        if nums.len() == 2 {
            test_cases.push((nums[0], nums[1]));
        } else {
            panic!("Each test case must contain exactly two numbers");
        }
    }

    // Process each test case
    for (l, r) in test_cases {
        // Replace this with your desired processing logic
        //println!("Processing range: {} to {}", l, r);

        /*
        Attempt 1
        let l_bin = to_binary(l);
        let r_bin = to_binary(r);
        println!("The binary representation of {} is {}", l, l_bin);
        println!("The binary representation of {} is {}", r, r_bin);
        let mut diff: u32 = r - l;
        let diff_bin = to_binary(diff);
        println!("The binary representation of {} is {}", diff, diff_bin);
        */

        /*
        Attempt 2

        let (mut pow2_bin, mut pow2) = max_power_of_2(l);
        //println!("pow2: {}, pow2_bin: {}", pow2, pow2_bin);

        let (_, pow2r) = max_power_of_2(r);
        if pow2 < pow2r {
            pow2 = l;
        }

        let upper = r - pow2;
        let lower = l - pow2;
        //println!("lower: {}, upper: {}", lower, upper);
        let up_bin = to_binary(upper);

        //println!("upper: {} upper bin: {}", upper, up_bin);

        let inv_bin = invert_bits(&up_bin);
        //println!("The inverted binary is {}", inv_bin);
        let mut inv = binary_to_u32(&inv_bin).unwrap();
        //println!("inv: {}", inv);

        let mut c: u32 = pow2 + inv;
        if inv == 0 {
            c = l + 1;
        }
        */

        /*
        Attempt 3. After reading editorial
        I kind of had that insight (two insights), but could not connect the two
        */

        let mut l_bin = to_binary(l);
        let r_bin = to_binary(r);

        // make it same length to compare
        l_bin = pad_binary(&l_bin, &r_bin);
        //println!("l_bin: {}, r_bin: {}", l_bin, r_bin);

        let n = l_bin.len();
        let mut k = n - 1;
        for i in 0..n {
            if l_bin.chars().nth(i) != r_bin.chars().nth(i) {
                break;
            }
            k -= 1;
        }
        //println!("most significant bit pos with difference: {}", k);

        let mut prefix: String = (&l_bin[0..n - k - 1]).to_string();
        //println!("prefix: {}", prefix);

        // append zeros:
        for i in 0..k + 1 {
            prefix.push('0');
        }

        //println!("after padding, prefix becomes: {}", prefix);
        let base = binary_to_u32(&prefix).unwrap();
        //println!("base additive: {}", base);

        let num1 = base + (1 << k); // 2^k
        let num2 = num1 - 1;
        let mut num3 = num1 + 1;
        if num3 > r {
            num3 = l;
        }

        println!("{} {} {}", num1, num2, num3);
        //println!("------");
    }
}

fn pad_binary(l_bin: &str, r_bin: &str) -> String {
    let l_len = l_bin.len();
    let r_len = r_bin.len();

    if l_len < r_len {
        let padding = "0".repeat(r_len - l_len);
        format!("{}{}", padding, l_bin)
    } else {
        l_bin.to_string() // Return l_bin as is if it's already the same length or longer
    }
}

fn to_binary(n: u32) -> String {
    format!("{:b}", n)
}

fn invert_bits(binary: &str) -> String {
    binary
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect()
}

fn binary_to_u32(binary: &str) -> Option<u32> {
    u32::from_str_radix(binary, 2).ok()
}

fn max_power_of_2(l: u32) -> (String, u32) {
    if l == 0 {
        return ("0".to_string(), 0);
    }

    // Find the largest power of 2 less than or equal to l
    let pow2 = 1 << (31 - l.leading_zeros());

    // Convert the value to a binary string
    let binary_representation = format!("{:b}", pow2);

    // Return the binary string and u32 value
    (binary_representation, pow2)
}
