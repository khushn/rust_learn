#[derive(Debug)]
enum MyList {
    Cons(i32, Box<MyList>),
    Nil,
}

use crate::MyList::{Cons, Nil};

// Reference counting Rc
#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

use crate::ListRc::{Cons as OtherCons, Nil as OtherNil};
use std::rc::Rc;

use std::cell::RefCell;

// Tree
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let b = Box::new(6);
    println!("b = {b}");

    // the print above works, but b is actually a pointer. Box::new() returns that
    assert_eq!(6, *b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // we can also do it like below. In that case the parent node will also be created on the heap
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    println!("list = {:?}", list);

    // dereference using '*'
    let x = 5;
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));

    // this is possible, i.e passing &m, instead of *m, because rust standard liberary
    // provided a deref of String to &str type
    hello(&m);

    // testing the Drop trait getting called
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // calling drop explicitly (it may be needed sometimes to release locks etc early)
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");

    // this is not allowed
    // c.drop();
    // instead call this
    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");

    // the Rc (reference count) lesson
    let a = Rc::new(OtherCons(5, Rc::new(OtherCons(10, Rc::new(OtherNil)))));
    println!(
        "Reference count after creating a = {}",
        Rc::strong_count(&a)
    );

    let b = OtherCons(3, Rc::clone(&a));
    println!("Rc --> b: {:?}", b);
    println!(
        "Reference count after creating b = {}",
        Rc::strong_count(&a)
    );

    {
        let c = OtherCons(4, Rc::clone(&a));
        println!("Rc --> c: {:?}", c);
        println!(
            "Reference count after creating c = {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "Reference count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );

    // Tree
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("branch node: {:?}", branch);
    println!("leaf node: {:?}", leaf);

    // macros (defined in lib.rs)
    let v: Vec<u32> = vec![1, 2, 3];

}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

// This is needed so that we can use the * (de reference) operator on the returned reference (pointer)
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Implementing Drop trait on smart pointers
// This is very much like smart pointers in C++
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
