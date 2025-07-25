fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    println!("Before defining closure: {:?}", list);

    // let only_borrows = || {
    //     println!("From closure: {:?}", list);
    // };

    let mut borrows_mutably = || list.push(6);

    // println!("Before calling closure: {:?}", list);

    // only_borrows();

    borrows_mutably();

    println!("After defining closure: {:?}", list);

}