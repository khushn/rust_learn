use std::io::{self, BufRead};

/*
Logic:
Loop 1 to n , just look for multiples of 5. but it was slow.


Improved logic: Just dicide by 5, then by 25, then by 125 ad so on till the divisor is below n

The reason it should work is 2s are always more than 5. And 2x5 create trailing zeros. Nothing less. 
Even numbers like 100, 1000 etc, yield to the above in this way.
*/
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut num_trailing_zeros: u32 = 0;
    let mut mult: u64 = 1;
    
        
    let mut divisor: u32 = 5;
    while divisor <= n {
    	num_trailing_zeros += n / divisor;
    	//println!("intermidiate val with divisor: {:?} is {:?}", divisor, num_trailing_zeros);
    	divisor *= 5;
    }
    

    println!("{:?}", num_trailing_zeros);
}
