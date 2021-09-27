fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, *y); wont work can eq different types and references is a different type

    let y = Box::new(x);
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    assert_eq!(5, *y);

    // implicit deref coercions

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
