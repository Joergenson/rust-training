enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))); // use box instead of recursive
}

// The Box<T> type is a smart pointer because it implements the Deref trait,
// which allows Box<T> values to be treated like references. When a Box<T> value
// goes out of scope, the heap data that the box is pointing to is cleaned up as
// well because of the Drop trait implementation
