
#[repr(C)]
pub struct Foo {
    x: u32
}

#[cbindgen_macro::export_c]
impl Foo {
    fn get(&self) -> u32 { self.x }
    fn get2(&self) -> u32 { self.x * 2 }
    fn get3(&self, a: u32) -> u32 { self.x  + a }
}

impl Drop for Foo {
    fn drop(&mut self) {

    }
}


#[test]
fn works() {
}