use std::slice;

fn main(){
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let mut safe_array = [0i32; 1000];
    let r = safe_array.as_mut_ptr();

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let values: &mut[i32] = unsafe {
        slice::from_raw_parts_mut(r, 1000)
    };

    println!("values is: {:?}", values);

}