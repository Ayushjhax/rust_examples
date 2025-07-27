use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    

    handle.join().unwrap();

    // by adding the move keyword, we are transferring ownership of v to the spawned thread
    // so we can't use v in the main thread

    // println!("{}", v[0]);
    // println!("{}", v[1]);
    // println!("{}", v[2]);
}