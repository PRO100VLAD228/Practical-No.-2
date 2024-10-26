use std::fmt::Display;

fn foobar<T: Display>(thing: T) {
    println!("{}", thing); // You can display it
}

fn main() {
    foobar(42); // Example with an integer
    foobar("Hello, World!"); // Example with a string slice
}
