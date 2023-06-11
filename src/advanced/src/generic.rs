struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn trans<U>(self, other: Point<U>) -> Point<U> {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn print() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p = p.trans(Point { x: 5.0, y: 10.0 });
    println!("p.distance_from_origin = {}", p.distance_from_origin());
}