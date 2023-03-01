
pub trait Retryable {
    fn retry(&self) -> bool;
}

impl Retryable for F {
    fn retry(&self) -> bool {
        *self
    }
}

struct Retry {}

// a func sum a + b
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", sum(1, 2));
    println!("Hello, world!");
}