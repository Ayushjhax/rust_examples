// fn bar -> ! {
//     loop {} 
// }

// fn main() {
//     let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
//     let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
//     let f: Box<dyn Fn(i32) -> i32> = Box::new(bar);

//     let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
//     let f: Box<dyn Fn(i32) -> i32> = Box::new(bar);

//     let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
//     let f: Box<dyn Fn(i32) -> i32> = Box::new(bar);

// }

impl <T> Option<T>{
    pub fn unwarp(self) -> T {
        match self {
            Some(x) => x,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

fn main() {
    // Example 1: Safe unwrap() - Some value
    let some_value: Option<i32> = Some(42);
    let unwrapped = some_value.unwrap(); // Returns 42
    println!("Unwrapped Some: {}", unwrapped);
    
    // Example 2: Dangerous unwrap() - None value
    let none_value: Option<i32> = None;
    // let unwrapped = none_value.unwrap(); // This would panic!
    
    // Example 3: Safer alternatives
    let x: Option<&str> = None;
    
    // Option 1: Use unwrap_or() to provide a default
    let safe_value = x.unwrap_or("default");
    println!("With unwrap_or: {}", safe_value);
    
    // Option 2: Use unwrap_or_else() with a closure
    let safe_value2 = x.unwrap_or_else(|| "computed default");
    println!("With unwrap_or_else: {}", safe_value2);
    
    // Option 3: Use match for explicit handling
    match x {
        Some(value) => println!("Got value: {}", value),
        None => println!("No value provided"),
    }
    
    // Example 4: Result unwrap()
    let result: Result<i32, &str> = Ok(100);
    let unwrapped_result = result.unwrap(); // Returns 100
    println!("Unwrapped Result: {}", unwrapped_result);
    
    // Example 5: Result with error
    let error_result: Result<i32, &str> = Err("something went wrong");
    // let unwrapped = error_result.unwrap(); // This would panic!
    
    // Safer Result handling
    match error_result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

// Demonstrating when unwrap() is acceptable
fn demonstrate_safe_unwrap() {
    // Safe: We know this will always be Some
    let config = Some("production");
    let env = config.unwrap(); // Safe because we control the value
    
    // Safe: After checking for None
    let maybe_value: Option<i32> = Some(42);
    if maybe_value.is_some() {
        let value = maybe_value.unwrap(); // Safe because we checked
        println!("Value: {}", value);
    }
}