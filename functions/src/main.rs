fn main() {
    println!("Hello, world!");
    let x = 6;
    another_function(x);
    println!("calling five() {}", five());

    let x = plus_one(12);

    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
	//x = 7;
	println!("another function x: {}!", x);
}

// last expression (without a semicolon is returned)
// NOTE: If you add a semicolon it becomes a statement
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}