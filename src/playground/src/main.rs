use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use crate::generic::{Composite, MyType};

mod bytes;
mod generic;
mod my_macro;
mod fibonacci;
mod associated_type;
mod underscore_variable;

fn main() {
    let mut result = TcpStream::connect("127.0.0.1:8002").unwrap();
    let _ = result.write_all(b"1");
}

fn buf_slice() {
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("{:?}", &buffer[1..]);
    println!("{:?}", &buffer[5..]);
    println!("{:?}", &buffer[8..]);
}