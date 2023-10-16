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


fn main() {
    create_cons_list_with_rc();
}
