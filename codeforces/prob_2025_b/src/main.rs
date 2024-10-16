use std::io;

fn main() {
    // Read first line to get N
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let N: usize = input.trim().parse().expect("Expected a number");

    // Read second line into nvals vector
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nvals: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    // Read third line into cvals vector
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let cvals: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();

    // Output the values (optional, for verification)
    //println!("N: {}", N);
    //println!("nvals: {:?}", nvals);
    //println!("cvals: {:?}", cvals);

    // implement 2^k modulo 10^9 + 7
    let mut ncp: Vec<u64> = Vec::with_capacity(100000);
    ncp.push(1);
    ncp.push(2);
    for i in 2..100000 {
        // Placeholder: Set the value for ncp[i]
        let modulus: u64 = 1_000_000_007;
        let value = (ncp[i-1] * 2) % modulus;
        ncp.push(value);
    }
    //println!("ncp: {:?}", ncp[99999]);

    for (index, &val) in cvals.iter().enumerate() {
        // Fill in your logic here
        //println!("Element at index {}: {}", index, val); // Example placeholder
        println!("{}", ncp[val as usize]);
    }
}
