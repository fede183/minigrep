use smart_pointers_box::cons_list::{create_cons_list};
use smart_pointers_box::my_box::{deref_test};
use smart_pointers_box::custom_smart_pointer::{create_and_drop, early_drop};

fn main() {
    create_cons_list();
    deref_test();
    create_and_drop();
    early_drop();
}
