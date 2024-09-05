fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third_elem: &i32 = &v[2];
    println!("The third element is {third_elem}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    v.push(6);

    // would give error because of the push above
    // can't have immutable borrow after the vector is mutated
    //println!("printng the third element again: {third_elem}");

    // we can change the values of all the elements
    for i in &mut v {
        *i += 50;
    }

    // iterate over all elements in a vector
    println!("iterate elements of vector");
    for i in &v {
        println!("{i}");
    }

    // To store values of different types in a vector, we use enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("iterate elements of vector having enums");
    for i in &row {
        println!("{:?}", i);
    }

    // part 2 -- String
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // the s1 reference is passed onto s (so unusable afterwards),
    // but s2 and s3 are fine to be used afterwards (internally this operator uses add() function )
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // internal representation of strings
    let hello = String::from("Hola");
    // below gives error as it stores as utf-8. and each position just has a byte its a vec<u8>
    //let answer = &hello[0];

    // part 3 -- Hash maps
    // its interesting that we can have `use` keyword anywhere
    // So sort of like Python that way!! -- but so different
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("hash map: {:?}", scores);

    // entry only sets if key not present
    scores.entry(String::from("Green")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Print scores again: {scores:?}");

    // use of or_insert to initialze with 0 val
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
