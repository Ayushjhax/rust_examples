use std::io;

fn main() {
    println!("Fahrenheit to Celsius Converter");
    println!("Enter temperature in Fahrenheit:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let fahrenheit: f64 = input.trim().parse()
        .expect("Please enter a valid number");
    
    let celsius = fahrenheit_to_celsius(fahrenheit);
    
    println!("{:.2}°F = {:.2}°C", fahrenheit, celsius);
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}