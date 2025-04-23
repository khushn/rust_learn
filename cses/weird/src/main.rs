use std::io;

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Parse the input as an integer
    let mut n: i64 = input.trim().parse().expect("Invalid integer input");
    let mut output = String::with_capacity(1000); // optional: estimate capacity


    // Your processing code here
    // For now, this is a sample processing that prints the Collatz sequence
    //let mut current = n;

    while n != 1 {
    	print!("{:?} ", n);
    	// not printing on every number as it causes a huge time in io
    	// output.push_str(&n.to_string());
    	// output.push(' ');

        if n % 2 != 0 {
            n = n * 3 + 1;
        } else {
            n /= 2;
        }

        
    }

    //output.push_str(&n.to_string());

    //println!("{}", output); // Print the final 1
    println!("{:?}", n);
}
