use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        // spawn = new thread
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // join the threads

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // using move closures

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // because v gets into the closure it can technicaly outlive the main function so for the closure to take ownership we use the move keyword
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); will make an error since v no longer is owned. in short trying to use a moved value

    handle.join().unwrap();
}
