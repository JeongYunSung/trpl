pub struct Node {
    value: usize,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop() {}", self.value);
    }
}

pub fn square() {
    let a = Node { value: 1 };
    let _a = Node { value: 2 };
    let _ = Node { value: 3 };

    println!("Hello, world!");
}