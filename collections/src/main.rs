enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Example 1: Creating and modifying a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Example 2: Accessing elements
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is: {}", third);
    let third_option: Option<&i32> = v.get(2);
    match third_option {
        Some(value) => println!("The third element (using get) is: {}", value),
        None => println!("There is no third element."),
    }

    // Example 3: Iterating over a vector
    for i in &v {
        println!("{}", i);
    }

    // Example 4: Mutating elements in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Mutated vector: {:?}", v);

    // Example 5: Using enum in a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Example 6: String operations
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s = String::from("hello");
    let s = s.replace("hello", "world");
    println!("{}", s);
}
