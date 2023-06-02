mod bindings;
use bindings::*;

#[test]
fn hi() {
    println!("hi");
    unsafe {
        println!("{:}", hello());
    }
}


