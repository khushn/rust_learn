use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        //println!("-----------");
        // Read the number of integers in the array for this test case
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Read the next line of integers into an array
        let arr: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Here, you can process `arr` as needed
        //println!("{:?}", arr); // Example: print the array

        if n == 1 {
            println!("1");
            continue;
        }

        // we tackle the case of n is even
        if n % 2 == 0 {
            let mut max1: usize = 0;
            for i in (0..n).step_by(2) {
                let dist = arr[i + 1] - arr[i];
                if dist > max1 {
                    max1 = dist;
                }
            }
            println!("{:?}", max1);
            continue;
        }

        // n is odd

        // find where to cut each array
        let mut max1: usize = arr[1] - arr[0];
        let mut prev1: usize = n-1;
        for i in 2..n {
            let dist = arr[i] - arr[i - 1];
            if dist > max1 {
                max1 = dist;
                prev1 = i - 1;
            }
        }
        //println!("prev1: {:?}, max1: {:?}", prev1, max1);

        let (mut b1, mut e1, mut b2, mut e2, mut b3, mut e3): (
            usize,
            usize,
            usize,
            usize,
            usize,
            usize,
        ) = (0, 0, 0, 0, 0, 0);

        // find the 2nd max for the odd side of the cut
        let mut beg: usize = 0;
        let mut end: usize = prev1 + 1;
        b1 = prev1 + 1;
        e1 = n-1;
        if (n - prev1) % 2 == 0 {
            beg = prev1 + 1;
            end = n;
            b1 = 0;
            e1 = prev1;
        }
        //println!("beg: {:?}, end: {:?}", beg, end);

        let mut max2: usize = 0;
        let mut prev2: usize = end-1;
        b2 = beg;
        e3 = end - 1;
        for i in beg..end - 1 {
            let dist = arr[i + 1] - arr[i];
            if dist > max2 {
                max2 = dist;
                prev2 = i;
            }
        }
        //println!("prev2: {:?}, max2: {:?}", prev2, max2);
        e2 = prev2;
        b3 = prev2 + 1;

        //println!("{:?}-{:?}, {:?}-{:?}, {:?}-{:?}", b1, e1, b2, e2, b3, e3);
        // TODO:

        // add max in the remaining 3 arrays, after 2 cuts.
        // array 1 range is 0..prev2+1
        // array 2 range is prev2+1..prev1+1
        // array 3 range is prev1+1..n
        let mut ans: usize = 0;
        for i in b1..e1 {
            let dist = arr[i + 1] - arr[i];
            if dist > ans {
                ans = dist;
            }
        }

        for i in b2..e2 {
            let dist = arr[i + 1] - arr[i];
            if dist > ans {
                ans = dist;
            }
        }

        for i in b3..e3 {
            let dist = arr[i + 1] - arr[i];
            if dist > ans {
                ans = dist;
            }
        }

        println!("{:?}", ans);
    }
}
