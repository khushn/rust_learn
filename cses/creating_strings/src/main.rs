use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    // Read a line from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim newline characters
    let input = input.trim();

    let mut map: HashMap<char, i32> = HashMap::new();
    for ch in input.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }

    println!("{:?}", map);

    let mut n: i32 = input.len() as i32;
    println!("n = {:?}", n);

    let mut num_permut: i32 = 1;

    for (k, v) in map.iter() {
        println!("k: {}, v: {}, num_permut: {} -- n: {}", k, v, num_permut, n);

        if n == *v {
            n -= *v;
            continue;
        }

        if *v == 1 {
            num_permut *= n;
            n -= *v;
            continue;
        }

        for i in (n - *v)..=n {
            num_permut *= i;
        }

        println!("after multiplying... num_permut: {}", num_permut);

        for i in 2..=*v {
            num_permut = num_permut / i;
        }

        println!("after dividing... num_permut: {}", num_permut);

        n -= v;
    }

    println!("{}", num_permut);
}
