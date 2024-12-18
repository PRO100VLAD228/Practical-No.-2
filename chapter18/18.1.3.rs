fn main() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable.clone());
    };

    consume();
    consume();
}

fn take<T>(_v: T) {}
