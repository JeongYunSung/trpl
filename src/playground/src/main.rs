use std::ops::Add;
use crate::generic::{Composite, MyType};

mod bytes;
mod my_macro;
mod generic;
mod associated_type;
mod underscore_variable;

fn main() {
    let num = four!();

    println!("{num}")
}

fn buf_slice() {
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("{:?}", &buffer[1..]);
    println!("{:?}", &buffer[5..]);
    println!("{:?}", &buffer[8..]);
}