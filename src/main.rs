#[link(name = "hello.dll", kind="dylib")]
extern {
    fn lib_test();
}

fn main() {
    unsafe {
        lib_test();
    }
}
