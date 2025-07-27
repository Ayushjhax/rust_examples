// Demonstration of Fn, FnMut, and FnOnce traits with closures in Rust
fn call_fn<F: Fn()>(f: F) {
    println!("Calling Fn closure:");
    f();
}

fn call_fnmut<F: FnMut()>(mut f: F) {
    println!("Calling FnMut closure:");
    f();
}

fn call_fnonce<F: FnOnce()>(f: F) {
    println!("Calling FnOnce closure:");
    f();
}

fn main() {
    let greeting = "hello";
    // Fn closure: only borrows, can be called multiple times
    let fn_closure = || println!("Fn: {}", greeting);
    call_fn(fn_closure);
    call_fn(fn_closure); // Can call again

    let mut count = 0;
    // FnMut closure: mutably borrows, can modify captured variables
    let mut fnmut_closure = || {
        count += 1;
        println!("FnMut: count is now {}", count);
    };
    call_fnmut(&mut fnmut_closure);
    call_fnmut(&mut fnmut_closure); // Can call again

    let name = String::from("Rust");
    // FnOnce closure: takes ownership, can only be called once
    let fnonce_closure = || {
        println!("FnOnce: moved name is {}", name);
        // name is moved here, so closure can only be called once
    };
    call_fnonce(fnonce_closure);
    // call_fnonce(fnonce_closure); // Uncommenting this will cause a compile error
}