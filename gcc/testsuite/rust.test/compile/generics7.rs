struct Foo<T>(T);

struct Bar {
    a: Foo<i32>,
    b: bool,
}

fn main() {
    let a = Foo::<i32>(123);
    let b: Bar = Bar { a: a, b: true };
    let c: i32 = b.a.0;
    // { dg-warning "unused name" "" { target *-*-* } .-1 }
}
