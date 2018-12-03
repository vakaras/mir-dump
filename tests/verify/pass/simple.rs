struct T {
    f: u32,
    g: u32,
}

fn foo(mut a: T) {
    let _x = a.f;
    let y = &mut a.g;
    let z = y;
    *z = 5;
    a.f = 6;
    assert!(a.f == 6 && a.g == 5);
}


fn main() {
    let a = T {
        f: 1,
        g: 2,
    };
    foo(a);
}
