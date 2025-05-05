use std::io;

fn main() {
    // Read the first line: number of elements
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Expected a number");

    // Read the second line: space-separated integers
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Expected an integer"))
        .collect();

    // Confirm input was correct
    if numbers.len() != n {
        eprintln!("Expected {} numbers, got {}", n, numbers.len());
        return;
    }

    // Example: print the numbers
    // println!("You entered: {:?}", numbers);

    /* debug code to check if i32 is sufficient or i64 is needed
    let mut sum: i64 = 0;
    for num in numbers {
        sum += num as i64;
    }
    println!("sum of numbers: {}", sum);
    */


    let ret = get_min_diff(&mut numbers, 0, 0, 0, n);
    println!("{}", ret);
}

fn get_min_diff(nums: &mut Vec<i32>, b1: i64, b2: i64, ind: usize, n: usize) -> i64 {
    if n - ind == 1 {
        let l = nums[ind] as i64;
        let diff1 = (b1 + l - b2).abs();
        let diff2 = (b2 + l - b1).abs();
        if diff1 <= diff2 {
            return diff1;
        } else {
            return diff2;
        }
    } 

    let l = nums[ind] as i64;
    let poss1 = get_min_diff(nums, b1 + l, b2, ind + 1, n);
    let poss2 = get_min_diff(nums, b1, b2 + l, ind + 1, n);
    if poss1 <= poss2 {
        return poss1;
    } else {
        return poss2;
    }
}
