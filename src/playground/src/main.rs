use std::ops::Add;
use crate::generic::{Composite, MyType};

mod bytes;
mod generic;
mod my_macro;
mod fibonacci;
mod associated_type;
mod underscore_variable;

fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n-2] + a[n-1]];

    for e in fib.take(10) { println!("{}", e) }
}

fn buf_slice() {
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("{:?}", &buffer[1..]);
    println!("{:?}", &buffer[5..]);
    println!("{:?}", &buffer[8..]);
}