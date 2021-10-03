fn main() {
    // conditional if let expressions

    let preffered_fruit: Option<&str> = Some("Strawberry");

    if let Some(fruit) = preffered_fruit {
        println!("Your preffered fruit! {}", fruit)
    }

    // while let / conditional loops

    let mut dummyStack = Vec::new();

    dummyStack.push(1);
    dummyStack.push(2);
    dummyStack.push(3);
    dummyStack.push(4);

    while let Some(end) = dummyStack.pop() {
        println!("Popping {}", end);
    }

    // for loops

    let list = vec!["apple", "strawberry", "potato"];

    for (index, value) in list.iter().enumerate() {
        println!("At index {} the value is {}", index, value);
    }
}
