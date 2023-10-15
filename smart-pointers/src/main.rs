use smart_pointers::cons_list::{create_cons_list};
use smart_pointers::cons_list_rc::{create_cons_list_with_rc};
use smart_pointers::my_box::{deref_test};
use smart_pointers::custom_smart_pointer::{create_and_drop, early_drop};

fn main() {
    create_cons_list();
    deref_test();
    create_and_drop();
    early_drop();
    create_cons_list_with_rc();
}
