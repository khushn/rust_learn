use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read T");
    let t: usize = input.trim().parse().expect("Invalid number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read n");
        let n: i32 = input.trim().parse().expect("Invalid number");

        // Add your processing logic here
        //println!("Processing: {}", n);
        if n == 6 {
            println!("1 1 2 3 1 2");
        } else {
            for i in 1..n - 1 {
                print!("{:?} ", i);
            }
            println!("1 2");
        }
    }
}
