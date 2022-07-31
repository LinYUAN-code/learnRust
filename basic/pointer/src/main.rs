use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(1, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }
    let c = Cons(4, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
}
