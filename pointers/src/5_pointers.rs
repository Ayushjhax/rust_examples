use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<RefCell<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<Rc<RefCell<List>>> {
        match self {
            List::Cons(_, next) => Some(Rc::clone(next)),
            List::Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(RefCell::new(List::Cons(
        Rc::new(RefCell::new(5)),
        Rc::new(RefCell::new(List::Nil)),
    )));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.borrow().tail());

    let b = Rc::new(RefCell::new(List::Cons(
        Rc::new(RefCell::new(10)),
        Rc::clone(&a),
    )));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.borrow().tail());

    if let Some(link) = a.borrow().tail() {
        *link.borrow_mut() = List::Cons(
            Rc::new(RefCell::new(15)),
            Rc::clone(&b),
        );
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

}