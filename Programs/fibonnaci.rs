use std::io;

fn main() {
    println!("Fibonacci Series Generator");
    println!("Enter the number of terms:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let n: usize = input.trim().parse()
        .expect("Please enter a valid number");
    
    println!("First {} terms of Fibonacci series:", n);
    
    // Method 1: Using a loop
    print_fibonacci_loop(n);
    
    println!("\n\nUsing recursive method:");
    print_fibonacci_recursive(n);
}

// Method 1: Iterative approach using a loop
fn print_fibonacci_loop(n: usize) {
    if n == 0 {
        return;
    }
    
    let mut a = 0u64;
    let mut b = 1u64;
    
    if n >= 1 {
        print!("{}", a);
    }
    if n >= 2 {
        print!(", {}", b);
    }
    
    for i in 2..n {
        let next = a + b;
        print!(", {}", next);
        a = b;
        b = next;
    }
    println!();
}

// Method 2: Recursive approach
fn fibonacci_recursive(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn print_fibonacci_recursive(n: usize) {
    for i in 0..n {
        if i == 0 {
            print!("{}", fibonacci_recursive(i));
        } else {
            print!(", {}", fibonacci_recursive(i));
        }
    }
    println!();
}