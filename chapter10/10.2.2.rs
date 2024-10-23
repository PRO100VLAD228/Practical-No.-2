fn print_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

fn main() {
    let arr = [1, 2, 3];
    print_array(&arr); 

    let arr = ["hello", "world"];
    print_array(&arr); 
}