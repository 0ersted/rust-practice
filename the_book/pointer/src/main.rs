enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::cell::RefCell;
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

fn linked_list_demo() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(NewList::Cons(Rc::clone(&value), Rc::new(NewList::Nil)));

    let b = NewList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = NewList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

use std::rc::Weak;
// tree structure
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn tree_demo() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );


    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    
    // set parent of leaf as branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
        );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );

    drop(branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        );

}

fn main() {
    // linked list
    linked_list_demo();

    // tree
    tree_demo();
}
