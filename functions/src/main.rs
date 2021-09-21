fn main() {
    
    // expression body
    let y = {
        let x = 3;
        x + 1 // no semicolon, expressions does not include semicolons since they dont return anything
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);
    
}

fn five() -> i32 {
    5
}