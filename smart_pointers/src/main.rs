#[derive(Debug)]
enum List {
    // Cons list using reference counting
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let myBox = Box::new(5);
    println!("myBox = {}", myBox);

    // Combining Rc and RefCell allows us to have a value with multiple owners
    // that you can mutate
    let value = Rc::new(RefCell::new(5));

    let a1 = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b1 = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a1));
    let c1 = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a1));

    *value.borrow_mut() += 10;

    println!("a1 after = {:?}", a1);
    println!("b1 after = {:?}", b1);
    println!("c1 after = {:?}", c1);

    // 
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // The below function uses deref coercion
    hello(&m);

    // Here, we investigate the drop trait
    let e = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");
    let f = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref allows us to customise the behaviour of dereferencing
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
