fn main() {
    let v: Vec<i32> = Vec::new(); // empty vec

    let v = vec![1, 2, 3]; // create filled vec

    let mut v = Vec::new(); // add elements to a vector
    v.push(5);
    v.push(6);
    v.push(7);

    println!("Value of v is {:?}", v);

    let third: &i32 = &v[2]; // get elements two ways
    println!("Third element is {}", third);

    match v.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("There is no such element"),
    }

    // iterate

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57]; // iterate mutable
    for i in &mut v {
        *i += 50;
    }

    // storing data in enums
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }

    // strings ---------

    let mut s = String::new();

    let s = String::from("initial contents");

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string(); // same as doing above

    // updating string

    let mut s = String::from("foo");
    s.push_str("bar"); // push string

    s.push('o'); // push single character

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // string slice
    let hello = "Здравствуйте";

    let s = &hello[0..4];


    // iterating strings

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


    // hashmaps

    use std::collections::HashMap;

    // create empty 
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // using iter function and collect
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

        let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // acces values

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // iterate

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating maps

    // overwriting

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // insert only if no key value

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // updae based on old value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    

    // do stuff with v
} // <- v goes out of scope and is freed here

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
