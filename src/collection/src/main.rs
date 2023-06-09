use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let x = scores.entry("Blue").or_insert(10);
    *x += 10;

    println!("{}", x);
}
