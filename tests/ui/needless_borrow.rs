#![feature(plugin)]
#![plugin(clippy)]

fn x(y: &i32) -> i32 {
    *y
}

#[deny(clippy)]
#[allow(unused_variables)]
fn main() {
    let a = 5;
    let b = x(&a);
    let c = x(&&a);
    let s = &String::from("hi");
    let s_ident = f(&s); // should not error, because `&String` implements Copy, but `String` does not
    let g_val = g(&Vec::new()); // should not error, because `&Vec<T>` derefs to `&[T]`
    let vec = Vec::new();
    let vec_val = g(&vec); // should not error, because `&Vec<T>` derefs to `&[T]`
    h(&"foo"); // should not error, because the `&&str` is required, due to `&Trait`
    if let Some(ref cake) = Some(&5) {}
}

fn f<T:Copy>(y: &T) -> T {
    *y
}

fn g(y: &[u8]) -> u8 {
    y[0]
}

trait Trait {}

impl<'a> Trait for &'a str {}

fn h(_: &Trait) {}

#[allow(dead_code)]
fn issue_1432() {
    let mut v = Vec::<String>::new();
    let _ = v.iter_mut().filter(|&ref a| a.is_empty());
    let _ = v.iter().filter(|&ref a| a.is_empty());

    let _ = v.iter().filter(|&a| a.is_empty());
}
