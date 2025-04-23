use std::io;

fn main() {
    // Read the first line: number of elements
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Invalid number");

    let mut nextodd: u32 = 0;
    let mut nexteven: u32 = 0;

    if n == 1 {
    	println!("1");
    	return;
    }

    if n < 4 {
        println!("NO SOLUTION");
        return;
    }

    if n % 2 == 0 {
        // even
        nexteven = n - 2;
        nextodd = n - 1;
    } else {
        nexteven = n - 1;
        nextodd = n - 2;
    }

    //println!("{:?}", nextodd);

    // print even in descending order
    for i in (2..=nexteven).rev().step_by(2) {
        print!("{:?} ", i);
    }

    // print the max number
    print!("{:?} ", n);

    // print the odd numbers in ascending order
    for i in (1..=nextodd).step_by(2) {
        print!("{} ", i); // Or print!("{:?} ", i); for space-separated output
    }

    println!("");
}
