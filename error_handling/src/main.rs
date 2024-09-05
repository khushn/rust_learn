use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    //v[10];

    let greeting_file_result = File::open("hello.txt");

    match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    let username = read_username_from_file_v2().unwrap();
    println!("user name: {:?}", username);

    let ch = last_char_of_first_line("\ndef");
    println!("last char of first line: {:?}", ch);

    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
	println!("IpAddr: {:?}", home);        

    // deliberate mis-spelling of file name to see error returned by main at thsi stage,
    // thanks to '?' operator use
    let uname = fs::read_to_string("hello.text")?;
    println!("uname : {uname}");

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// shorter version of the above use '?' operator to remove boiler plate code
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
