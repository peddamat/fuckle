#[link(name = "hello_lib.dll", kind="dylib")]
extern "C" {
    fn lib_test();
}

fn main() {
    unsafe {
        lib_test();
    }
}
