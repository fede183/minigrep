
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Reference
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target  = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str){
    println!("Hello, {}!", name);
}

pub fn deref_test() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Smart pointers
pub struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


pub fn create_and_drop() {
    let c = CustomSmartPointer {data: String::from("my stuff")};
    let d = CustomSmartPointer {data: String::from("other stuff")};
    println!("CustomSmartPointer created.");
}
