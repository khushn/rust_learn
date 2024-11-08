use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line to get n
    let n: usize = lines
        .next()
        .expect("Expected a line")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse n");

    // Read the second line to get n numbers
    let numbers: Vec<i64> = lines
        .next()
        .expect("Expected a line")
        .expect("Failed to read line")
        .split_whitespace()
        .take(n) // Ensure we only take n numbers
        .map(|num| num.parse().expect("Failed to parse a number"))
        .collect();

    // You can now add further processing on `n` and `numbers` here
    //println!("n = {}", n);
    //println!("numbers = {:?}", numbers);

    // Initialize another vector of the same size, filled with zeros
    let mut diffs: Vec<i64> = vec![0; n];

    // Set values in `other_numbers` using a loop
    for i in 0..n {
        diffs[i] = i as i64 - numbers[i]; // Example: setting each element to double of `numbers`
    }

    //println!("diffs = {:?}", diffs);

    let mut map: HashMap<i64, i64> = HashMap::new();
    for i in 0..n {
        let key = diffs[i];
        let value_to_add = numbers[i];

        // Use `entry` to add to an existing value or set it if the key doesn't exist
        map.entry(key)
            .and_modify(|v| *v += value_to_add)
            .or_insert(value_to_add);
    }

    //println!("map = {:?}", map);

    let mut max: i64 = 0;
    for (key, value) in &map {
    	if max < *value {
    		max = *value;
    	}
    }

    println!("{max}");
}
