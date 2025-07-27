// fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    // let a = 5;
    // let b = &a;

    // println!("a = {}", a);
    // println!("b = {}", *b);

    // let c = Box::new(a);

    // println!("c = {}", c);

//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// use std::ops::Deref;

// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }
