fn main() {
    println!("Hello, world!");
    println!("Feature A: {}", cfg!(feature = "A"));
    println!("Feature B: {}", cfg!(feature = "B"));
}
