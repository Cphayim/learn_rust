use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // if let Cons(x, _) = &b {
    //     *x.borrow_mut() += 10;
    // }

    increment(&b);

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn increment(list: &List<i32>) {
    if let Cons(x, sub_list) = &list {
        *x.borrow_mut() += 10;
        increment(&sub_list);
    }
}
