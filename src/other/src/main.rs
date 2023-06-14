static mut TEST: i32 = 5;

fn main() {
    unsafe {
        println!("{}", TEST);
        TEST += 1;
        println!("{}", TEST);
    }
}