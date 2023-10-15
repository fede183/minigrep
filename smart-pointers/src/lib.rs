pub mod cons_list {

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    pub fn create_cons_list() {
        let a = Cons(1, 
            Box::new(Cons(2, 
                    Box::new(Cons(3, 
                            Box::new(Nil)))
                    )
                )
            );
    }

}

pub mod cons_list_rc {
    // With Rc<T> we can multiple ownership of a data
    enum ListWithRc {
        Cons(i32, Rc<ListWithRc>),
        Nil,
    }

    use ListWithRc::{Cons, Nil};
    use std::rc::Rc;

    pub fn create_cons_list_with_rc() {
        let a = Rc::new(Cons(5, 
                Rc::new(Cons(10, 
                    Rc::new(Nil))
                    )
                )
            );
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating a = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

}

pub mod my_box {

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
}

pub mod custom_smart_pointer {


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


    use std::mem::drop;

    pub fn early_drop() {
        let c = CustomSmartPointer {data: String::from("some data") };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }

}
