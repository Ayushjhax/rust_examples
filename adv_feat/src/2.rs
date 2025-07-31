use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);
    println!("a: {:?}, b: {:?}", a, b);

    // let (a, b) = split_at_mut(v, 3);
    // println!("a: {:?}, b: {:?}", a, b);
}