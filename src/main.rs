// mod lib;
// use crate::lib::*;

#[link(name = "fart.dll", kind="dylib")]
extern "C" {
    fn lib_test();
}

fn main() {
    unsafe {
        lib_test();
    }
}
