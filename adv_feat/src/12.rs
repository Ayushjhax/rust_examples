#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3, 4, 5, 5];
    println!("{:?}", v);
    
    // Compare with built-in vec! macro
    let built_in_v = vec![1, 2, 3, 4, 5, 5];
    println!("Built-in vec!: {:?}", built_in_v);
    
    // Test with different types
    let string_vec = my_vec!["hello", "world", "rust"];
    println!("String vector: {:?}", string_vec);
}