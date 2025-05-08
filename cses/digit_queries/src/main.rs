use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of queries
    let q: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Invalid number of queries"),
        _ => {
            eprintln!("No input found");
            return;
        }
    };

    let mut queries: Vec<u64> = Vec::with_capacity(q);

    for _ in 0..q {
        if let Some(Ok(line)) = lines.next() {
            let value: u64 = line.trim().parse().expect("Invalid query input");
            queries.push(value);
        } else {
            eprintln!("Not enough input lines for queries");
            return;
        }
    }

    // Output to verify
    //println!("Number of queries: {}", q);
    //println!("Query values: {:?}", queries);

    for q in queries {
        let mut slot: u64 = 0;
        let mut rem_digits = q;
        for i in 0..19 {
            slot = i as u64;
            let sub = 9 * 10u64.pow(i) * (i + 1) as u64;
            //println!("sub: {}", sub);
            if rem_digits > sub {
                rem_digits -= sub;
            } else {
                break;
            }
        }

        let beg_range = 10u64.pow(slot as u32);
        /*
        println!(
            "rem_digits: {}, slot: {}, beg_range: {}",
            rem_digits, slot, beg_range
        );
        */

        // (rem_digits - 1) to zero index it
        let target_num = beg_range + (rem_digits - 1) / (slot + 1);
        let rem_num_digits = (rem_digits - 1) % (slot + 1);

        /*
        println!(
            "target_num: {}, rem_num_digits: {}",
            target_num, rem_num_digits
        );
        */

        let mut ans = target_num;
        /*
        if rem_num_digits == 0 {
            ans += 1;
        }
        */
        //println!("ans: {}", ans);

        let ans_str = ans.to_string();
        println!("{}", ans_str.chars().nth(rem_num_digits as usize).unwrap());
        /*
        if rem_num_digits == 0 {
            println!("{}", ans_str.chars().nth(slot as usize).unwrap());
        } else {
            println!("{}", ans_str.chars().nth(rem_num_digits as usize).unwrap());
        }
        */
        //println!("----");
    }
}
