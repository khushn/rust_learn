fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer.x = {}", integer.x());
    println!(
        "float.distance_from_origin() = {}",
        float.distance_from_origin()
    );

    let s = "abcdef";
    let t = "abc";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "abcdef";
    let t = "bcd";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "abcdefghabkl";
    let t = "abk";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "abc";
    let t = "abk";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "abc";
    let t = "abkkhd";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "abc";
    let t = "";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "abc";
    let t = "c";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "abc";
    let t = "a";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );
    let s = "ttttd";
    let t = "ttd";
    println!(
        "isSubstring:(\"{}\", \"{}\") -> {}",
        s,
        t,
        isSubstring(s, t)
    );

    // invoking traits feature
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    notify2(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        ),
    };

    notify3(&article, &tweet);

    // calling function which uses lifetimes
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let i;
    {
        let first_sentence = novel.split('.').next().unwrap();
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("ImportantExcerpt: {i:?}");
}

struct Point<T> {
    x: T,
    y: T,
}

// this method applicable for all point types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This method will only be applicable for f32 point types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn isSubstring(s: &str, t: &str) -> bool {
    let n = s.len();
    let m = t.len();
    let mut i = 0;
    let mut j = 0;
    let mut match_len = 0;

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    while i < n && j < m {
        if s_chars[i] == t_chars[j] {
            j += 1;
            match_len += 1;
            if match_len == m {
                return true;
            }
        } else {
            i -= match_len;
            match_len = 0;
            j = 0;
        }
        i += 1;
    }
    false
}

// Traits
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Take trait as a parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// another way to declare it
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Taking 2 different implementations
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// lifetimes annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct with lifetime annotated field
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    // in this case the below field will always hold a reference to some variable
    // and strcut life should be lesser than that variable's life time denoted by 'a
    part: &'a str,
}


// The liftimes used in methods on the struct 
// rule 2 and rule 3 mentioned in lifetimes sub chapter 10.3 (towards latter part) apply
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}


/*
// largest function written using generics
fn largest_g<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/
