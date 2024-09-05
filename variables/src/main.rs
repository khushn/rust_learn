const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    //x = 5;

    // shadows prev variable x
    let x = x + 1;

    {
        // shadows outer x (scope only within containing braces)
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // char types
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // Tuples
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", tup.1);
    println!("The value of tuple is: {:?}", tup);

    // arrays
    let a = [1, 2, 3, 4, 5];
    println!("The value of array a is: {:?}", a);
}
