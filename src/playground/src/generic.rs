use std::fmt::{Display, Formatter};
use std::ops::Add;

pub struct MyType<T> {
    pub value: T,
}

impl <T: Display> MyType<T> {
    pub fn print(&self) {
        println!("print : {}", self.value);
    }
}

impl <T: Add> MyType<T> {
    pub fn print_add(&self) {
        println!("add");
    }
}

impl <T: Display + Add> MyType<T> {
    pub fn two(&self) {
        println!("two");
    }
}

pub struct Composite {
    pub value: i32,
}

impl Add for Composite {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self
    }
}

impl Display for Composite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Composite : {}", self.value)
    }
}