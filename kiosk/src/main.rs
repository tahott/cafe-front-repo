use libs::rand::{self, Rng};
fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();

    println!("random number is {}", rng.gen_range(1..=10));
}
