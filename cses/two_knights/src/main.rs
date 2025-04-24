use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut vec: Vec<u64> = vec![0; n];

    for r in 1..=n {
        //println!("{:?}", i);
        for c in 1..=n {
            if r + 1 <= n && c + 2 <= n {
                let max = get_max(r + 1, c + 2);
                vec[max - 1] += 1;
            }

            if r + 2 <= n && c + 1 <= n {
                let max = get_max(r + 2, c + 1);
                vec[max - 1] += 1;
            }

            if r + 1 <= n && c >= 3 {
                let max = get_max(r + 1, c);
                vec[max - 1] += 1;
            }

            if r + 2 <= n && c >= 2 {
                let max = get_max(r + 2, c);
                vec[max - 1] += 1;
            }
        }
    }

    //println!("{:?}", vec);

    // put cumulatives in vec
    for i in 1..n {
        vec[i] += vec[i - 1];
    }

    //println!("cumulatives vals: {:?}", vec);

    // print answers subtracting vec index val from N-c-R
    for i in 1..=n {
        if i == 1 {
            println!("0");
            continue;
        }

        let isq = i * i;
        let ans = (isq * (isq - 1) / 2) as u64 - vec[i - 1];
        println!("{:?}", ans);
    }
}

fn get_max<T: Ord>(x: T, y: T) -> T {
    if x <= y {
        y
    } else {
        x
    }
}
