fn main() {
    println!("Hello, world!");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn internal_test_add() {
        let test = "tests";
        assert_eq!(test, "tests", "Test Result : {}", test);
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Make this tests fail");
    }
}
