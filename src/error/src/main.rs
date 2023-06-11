use std::fs::File;
use std::io::{self, Read};

fn main() {
    panic!("crash and burn");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

impl From<io::Error> for OurError {
    fn from(err: io::Error) -> OurError {
        OurError::Io(err)
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
