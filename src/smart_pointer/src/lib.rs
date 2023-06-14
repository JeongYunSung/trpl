use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn print_rc_pointer() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub fn print_ref_cell_pointer() {
    let rec = RefCell::new(String::from("Hello"));

    rec.borrow_mut().push_str(" World");
    println!("{}", rec.into_inner());
}