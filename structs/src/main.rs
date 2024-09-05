// This stereotype is to be able to print the struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// adding methods to area struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // asscociated functions (distinct than methods above in that they don't take 'self')
    // similar to static class methods in C++
    fn square(size: u32) -> Self {
        let s = Self {
            width: size,
            height: size,
        };
        s
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user1: {:?}", user1);

    // easy way to create a new instance, which has all other fields copied
    // but one or few different
    let user2 = User {
        email: String::from("yet_another@example.com"),
        username: user1.username.clone(), // this is needed for uuser1 to be useful
        ..user1
    };

    println!("user2: {:?}", user2);

    // user1 becomes unusable because of username String pointer was assigned to user2
    // but this now works as I used clone() when copying
    println!("user1: {:?}", user1);

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect1,
        area(&rect1)
    );

    println!("checking validity of rect1 : {:?}", rect1);

    // we can also use the dbg! macro for printing
    dbg!(&rect1);

    dbg!(&rect1.area());

    let rect2 = Rectangle {
        width: 29,
        height: 50,
    };

    dbg!(rect1.can_hold(&rect2));
    dbg!(rect2.can_hold(&rect1));

    dbg!(Rectangle::square(4));
}

// Way to reduce typing field names repeatedly
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    //rectangle.width -=1;
    rectangle.width * rectangle.height
}
