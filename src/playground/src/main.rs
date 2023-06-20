use std::ops::Add;
use crate::generic::{Composite, MyType};

mod bytes;
mod generic;
mod associated_type;
mod underscore_variable;

fn main() {
    let my_type1 = MyType {
        value: 1
    };

    my_type1.print_add();

    let my_type2 = MyType {
        value: "hello"
    };

    my_type2.print();

    let my_type3 = MyType {
        value: Composite {
            value: 1
        }
    };

    my_type3.print_add();
    my_type3.print();
}

fn buf_slice() {
    let buffer: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("{:?}", &buffer[1..]);
    println!("{:?}", &buffer[5..]);
    println!("{:?}", &buffer[8..]);
}