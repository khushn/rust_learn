use std::collections::HashMap;
use std::io;

fn main() {
    // Read the number of test cases
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let T: usize = input.trim().parse().expect("Expected a number");

    // Loop through each test case
    for _ in 0..T {
        // Read the values of n and k
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let (n, k): (usize, usize) = {
            let parts: Vec<usize> = input
                .split_whitespace()
                .map(|s| s.parse().expect("Parse error"))
                .collect();
            (parts[0], parts[1])
        };

        // Read the n numbers into a vector
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let mut numbers: Vec<u64> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error"))
            .collect();

        // Placeholder: Write your logic here
        //println!("n: {}, k: {}", n, k);
        //println!("Numbers: {:?}", numbers);

        numbers.sort();
        //println!("Sorted Numbers: {:?}", numbers);

        // 1. Extract unique numbers (since the vector is sorted)
        let mut unique_numbers = numbers.clone(); // Clone the original vector
        unique_numbers.dedup(); // Remove consecutive duplicates
        //println!("Unique numbers in ascending order: {:?}", unique_numbers);

        // 2. Create a HashMap to count occurrences
        let mut counts: HashMap<u64, u64> = HashMap::new();
        for &num in &numbers {
            *counts.entry(num).or_insert(0) += 1;
        }

        // Output the map of number counts
        //println!("Number counts: {:?}", counts);

        let mut max = 0;
        let mut cur_max = 0;
        let mut cur_beg = 0;
        for (index, &num) in unique_numbers.iter().enumerate() {
            //println!("Index: {}, Unique num: {}", index, num);

            if index == 0 {
                cur_max = counts[&num];
                continue;
            }

            let diff = index - cur_beg;
            if diff >= k {
                if cur_max > max {
                    max = cur_max;
                }

                // reduce the left most count
                cur_max -= counts[&unique_numbers[cur_beg]];
                cur_beg = cur_beg+1;
                cur_max += counts[&num];
                continue;
            }

            let prev = unique_numbers[index - 1];
            if *&num > prev + 1 {
                if cur_max > max {
                    max = cur_max;
                }
                cur_beg = index;
                cur_max = counts[&num];
                continue;
            }

            cur_max += counts[&num];
        }

        if cur_max > max {
            max = cur_max;
        }

        println!("{}", max);
    }
}
