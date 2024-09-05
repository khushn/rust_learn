#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home ip: {:?}", &home);
    dbg!(&loopback);

    // Option enum -- for handling null values
    // Option<T> values
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    dbg!(some_number);
    dbg!(absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // gives error
    // let sum = x + y;
    // so we have to use unwrap()
    let sum = x + y.unwrap();

    dbg!(sum);

    // match with enum
    let c = Coin::Nickel;
    dbg!(value_in_cents(&c));

    let x: f32 = value_in_cents(&c).into();
    dbg!(x);

    // checking for None values
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
    dbg!(none);

    // catch all place holder in match
    // use of 'other'
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Use of If let instead of the verbose match
    let mut count = 0;
    let c2 = Coin::Quarter(State::California);
    if let Coin::Quarter(state) = c2 {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

#[derive(Debug)]
enum State{
	Alaska, 
	California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

// checking for None values
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
