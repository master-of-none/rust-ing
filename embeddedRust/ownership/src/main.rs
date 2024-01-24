#[derive(Debug)]

struct S<'a>(&'a mut u8);

impl S<'_> {
    fn new(v: &mut u8) -> S {
        S(v)
    }
}
fn main() {
    let mut zero = 0;
    let mut x = S::new(&mut zero);
    let y = &mut x;
    *y.0 = 1;
    let z = y;
    println!("{:?}", *z);
    println!("{:?}", x);
}
