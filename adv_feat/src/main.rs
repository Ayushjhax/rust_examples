unsafe fn dangerous(){
    println!("I'm dangerous");
}

fn main (){
    let mut v = vec![1,2,3,4,5];
    let ref mut r = v;
    let r = r.as_mut_slice();
    r[0] = 10;
    println!("{:?}", v);

    unsafe {
        dangerous();
    }
}