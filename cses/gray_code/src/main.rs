use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut vec: Vec<String> = Vec::new();
    vec.push("0".to_string());
    vec.push("1".to_string());

    for i in 1..n {
    	vec = double_vec(vec);
    	
    }

    //println!("{:?}", vec);

    for v in vec {
    	println!("{}", v);
    }
}


fn double_vec(input: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    // Add your processing logic here
    // For example, you might iterate through the input vector:
    for s in &input {
        result.push("0".to_string() + &s.clone());
    }

    for s in input.iter().rev() {
    	result.push("1".to_string() + s);
    }

    // After processing, return the 'result' vector
    result
}