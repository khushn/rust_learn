use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: u64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let sum: u64 = n * (n + 1) / 2;

    if sum % 2 != 0 {
        println!("NO");
    } else {
        println!("YES");

        let mut excl: u64 = 0;
        let mut first_set_end: u64 = 0;
        let mut lsum: u64 = 0;
        let mut lset_count: u64 = 0;
        for i in 1..n {
            lsum += i;
            lset_count += 1;
            if lsum == sum / 2 {
                first_set_end = i;
                break;
            } else if lsum > sum / 2 {
                excl = lsum - sum / 2;
                lset_count -= 1;
                first_set_end = i;
                break;
            }
        }

        // print left set
        println!("{:?}", lset_count);
        for i in 1..=first_set_end {
            if i != excl {
                print!("{:?} ", i);
            }
        }
        println!("");

        // print right set
        println!("{:?}", n - lset_count);
        if excl > 0 {
        	print!("{:?} ", excl);
        }
        for i in (first_set_end + 1)..=n {
            print!("{:?} ", i);
        }
        println!("");
    }
}
