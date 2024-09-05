fn main() {
    let s1 = String::from("hello");

    // value on the heap moves to s2
    let s2 = s1;

    // so this gives error, as s1 is no longer a valid variable
    // println!("{s1}, world!");

    // but this works
    println!("{s2}, world!");

    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let _s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(_s2); // s2 is moved into
                                         // takes_and_gives_back, which also
                                         // moves its return value into s3

    // This will give error as _s2 is no longer in scope
    // println!("_s2: {_s2}");

    println!("_s3: {_s3}");

    // Note previous s1 is freed. But that's fine
    // we can reuse (bit unintuitive. But need to understand from memory prevention view)
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    // better way of calling
    let s1 = String::from("hello");
    let len = calculate_length_2(&s1);
    println!("The length of '{s1}' is {len}.");

    // Try to modify the  value pointed by the reference
    let mut s = String::from("hello");

    change(&mut s);
    println!("s: {s}");

    // lesson - there can be only one reference to a mutable object
    // let mut s = String::from("hello");

    //let r1 = &mut s;
    //let r2 = &mut s;

    // will give error
    //println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    let s4 = no_dangle();
    println!("s4: {s4}");

    // From chapter section 4.3 on slices
    let mut s = String::from("hello world");

    //let word = first_word_2(&s); // word will get the value 5

    let word = first_word_3(&s); // will return a slice

    //s.clear(); // this empties the String, making it equal to ""

    //let s2 = &s[0..word];

    println!("s: {s}, first word: {word}");

    let my_string_literal = "yello world";
    let word = first_word_3(&my_string_literal);
    println!("my_string_literal: {s}, first word: {word}");

    // slice on ints
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    println!("slice of ints: {:?}", slice);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// This is a better way. Just passing reference to the string
fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
// Gives error
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

// no error
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// better than the above one as it takes in a slice, where String also can be passed
fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
