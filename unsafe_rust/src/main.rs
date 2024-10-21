extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // 19.1 unsafe code

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // *r1 = 6;
        println!("r1 is: {}", *r1);

        *r2 = 7;
        println!("r2 is: {}", *r2);
        println!("r1 is: {}", *r1);
    }

    println!("num is: {}", num);

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // sectiom 19.2 Advanced traits
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // calling same method name on a struct implementing differet traits
    // even having one implementation of the method directly on the struct
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // super trait
    let p = Point { x: 2, y: 3 };
    p.outline_print();

    // section 19.3 Advanced types

    // also called as newtypes,
    // basically a type alias to give it meaning to API callers for example
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // to have shorter aliases for long and complex types
    // e.g in this case a closure function variable which lives through out the entirety of the program
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        f();
    }

    fn returns_long_type() -> Thunk {
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
        f
    }

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi there!!"));
    takes_long_type(f);

    // section 19.4
    // function pointers
    println!("----function pointers passing to another function----");
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}

// advaned traits
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Add on a different (non default) type Meters
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Rust allows same method names on different traits
// possible for a struct to even implement different traits having same method name
// and to call them
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Super trait
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// section 19.4
// function pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
