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

    // println!("{:?}", input);

    let mut char_count: HashMap<char, i32> = HashMap::new();

    for ch in input.chars() {
        //println!("{:?}", ch);
        *char_count.entry(ch).or_insert(0) += 1;
    }

    //println!("char_count: {:?}", char_count);

    let mut num_odd_count: i32 = 0;
    let mut odd_chars = String::new();
    let mut prefix = String::new();
    for (ch, count) in &char_count {
        if count % 2 != 0 {
            num_odd_count += 1;
            for i in 0..*count {
                odd_chars.push(*ch);
            }
        } else {
            for i in 0..count / 2 {
                prefix.push(*ch);
            }
        }
    }

    if num_odd_count > 1 {
        println!("NO SOLUTION");
    } else {
        //print!("{:?}", prefix);
        if num_odd_count == 1 {
            //print!("{:?}", odd_chars);
        }
        let reversed: String = prefix.chars().rev().collect();
        //println!("{:?}", reversed);

        let concat = prefix + &odd_chars + &reversed;
        println!("{}", concat);
    }
    // You can now process `input` as a &str
    // Example: println!("You entered: {}", input);
}
