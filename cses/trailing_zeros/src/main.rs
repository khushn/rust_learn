use std::io::{self, BufRead};

/*
Logic:
Loop 1 to n , remove trailing zeros of each no. and add to count.
Preserve only the last digit after multiplying. If multiplication results in a new trailing zero,
account for it and remove it
*/
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: u32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // let MODULO: u32 = 1000000000;
    let MODULO: u32 = 1000;

    let mut num_trailing_zeros: u32 = 0;
    let mut mult: u64 = 1;
    for num in 2..=n {
        //println!("processing num: {:?}", num);
        let mut i = num;
        while (i / 10) * 10 == i {
            i = i / 10;
            num_trailing_zeros += 1;
        }

        //println!("num_trailing_zeros in {:?} : {:?}", i, num_trailing_zeros);

        i = i % MODULO;
        //println!("last non zero digit: {:?}", i);
        mult *= i as u64;
        if (mult / 10) * 10 == mult {
            mult /= 10;
            num_trailing_zeros += 1;
        }

        /*
        println!(
            "num_trailing_zeros after mult {:?} : {:?}",
            mult, num_trailing_zeros
        );
        */

        mult = mult % MODULO as u64;
        //println!("residual mult: {:?}", mult);
        //println!("---");
    }

    println!("{:?}", num_trailing_zeros);
}
