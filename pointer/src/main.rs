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
        let c = Cons(4, a.clone()); 
        assert_eq!(Rc::strong_count(&a), 3);
    }

    assert_eq!(Rc::strong_count(&a), 2);
    drop(b);
    assert_eq!(Rc::strong_count(&a), 1);

}
