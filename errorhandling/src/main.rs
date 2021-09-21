use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("Crash");

    let v = vec![1, 2, 3];

    // v[99];

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = File::open("hello.txt").unwrap(); // calls panic if result is Err else returns content of Ok

}

use std::error::Error;

fn read() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?; // ? returns the value of the result if ok it will return that value else it will return the nature of the error

    Ok(())
}


// a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

// The bad state is not something that’s expected to happen occasionally.
// Your code after this point needs to rely on not being in this bad state.
// There’s not a good way to encode this information in the types you use.