struct S {}

trait T {
    fn foo(&self);
}

impl T for S {
    fn foo(&self) {}
}

fn main() {
    let s = S {};
    s.foo();
}
