use add_one;
use add_two;

fn main() {
    let x = 10;
    let y = 20;
    println!("add one : {}", add_one::add_one(x));
    println!("add two : {}", add_two::add_two(x, y));
}