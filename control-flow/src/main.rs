fn main() {
    //if expressions
    let number = 3;

    if number < 5 {
        println!("Condition was true")
    } else {
        println!("Condition was false")
    }

    //else if

    let number = 4;

    if number % 4 == 0 {
        println!("Number is divisible by 4")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3")
    } else if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("Number is not divisible by 4, 3, or 2")
    }

    //if statement in variabe declaration
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number three is: {}", number);

    //loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // loops with while 

    let mut number = 4;

    while number != 0 {
        println!("{}!",number);
        number -= 1 ;
    }

    println!("Collection with while loop:");

    
    // collection using while
    let a = [10,20,30,40,60];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // collection using for loop

    println!("For loop:");

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // collection using for loop, range and reverse
    
    println!("Collection using for loop, range and reverse:");

    for number in (0..a.len()).rev() {
        println!("The value is: {}", a[number]);
    }


}
