use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The formatter 'f' is where we write our formatted text
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Let's also implement Debug to see the difference
impl fmt::Debug for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Debug formatting shows more detail
        write!(f, "Wrapper({:?})", self.0)
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    
    // Display trait - uses our custom formatting
    println!("Display: {}", w);
    
    // Debug trait - shows more detailed structure
    println!("Debug: {:?}", w);
    
    // Let's see what the formatter can do
    demonstrate_formatter();
}

fn demonstrate_formatter() {
    println!("\n=== Formatter Examples ===");
    
    // The formatter can handle different types of formatting
    let number = 42;
    let text = "hello";
    
    // Basic formatting
    println!("Number: {}", number);
    println!("Text: {}", text);
    
    // Width and alignment
    println!("Number with width: {:>5}", number);  // Right-aligned, 5 chars wide
    println!("Number with width: {:<5}", number);  // Left-aligned, 5 chars wide
    println!("Number with width: {:^5}", number);  // Center-aligned, 5 chars wide
    
    // Zero-padding
    println!("Zero-padded: {:05}", number);
    
    // Precision for floating point
    let pi = 3.14159;
    println!("Pi with 2 decimal places: {:.2}", pi);
    
    // Different number formats
    println!("Binary: {:b}", number);
    println!("Hexadecimal: {:x}", number);
    println!("Octal: {:o}", number);
} 