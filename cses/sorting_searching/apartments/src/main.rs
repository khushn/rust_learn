use std::io;

fn main() {
    // Read first line: n, m, k
    let mut first_line = String::new();
    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read line");
    let mut nums = first_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Expected an integer"));

    let n = nums.next().unwrap();
    let m = nums.next().unwrap();
    let k = nums.next().unwrap() as u32;

    // Read second line: n integers
    let mut second_line = String::new();
    io::stdin()
        .read_line(&mut second_line)
        .expect("Failed to read line");
    let mut a: Vec<u32> = second_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Expected a number"))
        .collect();
    assert_eq!(a.len(), n);

    // Read third line: m integers
    let mut third_line = String::new();
    io::stdin()
        .read_line(&mut third_line)
        .expect("Failed to read line");
    let mut b: Vec<u32> = third_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Expected a number"))
        .collect();
    assert_eq!(b.len(), m);

    // Sort both vectors
    a.sort();
    b.sort();

    // For verification
    // println!("Sorted a: {:?}", a);
    // println!("Sorted b: {:?}", b);

    let mut bind: usize = 0;
    let mut num_alloted: u32 = 0;
    for a1 in a {
        if bind >= b.len() {
            break;
        }

        let mut b1 = b[bind];
        while b1 + k < a1 && bind < b.len() {
            bind += 1;
            if bind == b.len() {
            	continue
            }
            b1 = b[bind];
        }

        if b1 > a1 + k {
            continue;
        }

        if b1 + k >= a1 && b1 <= a1 + k {
            num_alloted += 1;
            bind += 1;
            continue;
        }
    }

    println!("{}", num_alloted);
}
