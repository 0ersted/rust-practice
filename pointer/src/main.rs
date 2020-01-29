enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

#[test]
fn reference_count_test() {
    let a = Rc::new(Cons(5, Rc::new(   // rc of list
                Cons(10, Rc::new(Nil))
            )));

    assert_eq!(Rc::strong_count(&a), 1);

    let b = Cons(3, Rc::clone(&a));
    assert_eq!(Rc::strong_count(&a), 2);

    {
        // not prefered by Rust for not explicitly creating a rc
        let _c = Cons(4, a.clone()); 
        assert_eq!(Rc::strong_count(&a), 3);
    }

    assert_eq!(Rc::strong_count(&a), 2);
    drop(b);
    assert_eq!(Rc::strong_count(&a), 1);

}

#[derive(Debug)]
// a list having multiple owners of mutable data
enum NewList {
    Cons(Rc<RefCell<i32>>, Rc<NewList>),
    Nil,
}

use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(NewList::Cons(Rc::clone(&value), Rc::new(NewList::Nil)));

    let b = NewList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = NewList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

