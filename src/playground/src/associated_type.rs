use std::ops::Add;

#[derive(Debug)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug)]
pub struct  Vector {
    pub x: u8,
    pub y: u8,
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        println!("Add Point, {:?}, {:?}", self, other);
        self
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        println!("Add Vector, {:?}, {:?}", self, other);
        self
    }
}