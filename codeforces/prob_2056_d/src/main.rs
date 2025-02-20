use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read T");
    let t: usize = input.trim().parse().expect("Invalid number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read n");
        let n: usize = input.trim().parse().expect("Invalid number");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read array");
        let arr: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        // Add your processing logic here
        //println!("Processing array: {:?}", arr);

        let mut ctr: Vec<i32> = vec![0; 10];
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; 10];

        for i in 0..n {
            let v = arr[i] as usize;
            ctr[v - 1] += 1;
            for j in 0..10 {
                dp[j][i] = ctr[j];
            }
        }

        //println!("{:?}", dp); // Prints the 2D array
        let mut sum: i64 = 0;

        // for each subarray size
        for i in 1..n + 1 {
            if i % 2 != 0 {
                // odd
                sum += (n - i + 1) as i64;
            } else {
                // even size, we need to check
                for j in 0..n - i + 1 {
                    let mut k = j + i - 1;
                    //println!("{:?}, {:?}", j, k);
                    let res_col = subtract_columns(&dp, j as i32 - 1, k as i32);
                    //println!("res_col: {:?}", res_col);

                    // compare the ith and (i+1)th element in res_col,
                    // if they are equal
                    // we can increase the sum

                    let mut rem = i as i32 / 2 + 1;
                    for ind in 0..res_col.len() {
                        let num = res_col[ind];
                        if num >= 2 && num >= rem {
                            // equal found
                            /*
                            println!(
                                "equal found at index: {:?}, value: {:?}, i: {:?}, rem: {:?}",
                                ind, num, i, rem
                            );
                            */
                            sum += 1;
                            break;
                        } else {
                            rem -= num;
                            if rem <= 0 {
                                break;
                            }
                        }
                    }
                }
            }
        }

        println!("{:?}", sum);
    }
}

fn subtract_columns(arr: &Vec<Vec<i32>>, col1: i32, col2: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for row in arr.iter() {
        let mut val = row[col2 as usize];
        if col1 >= 0 {
            val = row[col2 as usize] - row[col1 as usize];
        }
        result.push(val); // Example: Sum of values at col1 and col2
    }

    result
}
