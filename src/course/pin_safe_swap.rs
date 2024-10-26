use std::marker::PhantomPinned;
use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(text: &str) -> Pin<Box<Self>> {
        let test = Test {
            a: String::from(text),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };

        let mut boxed = Box::pin(test);
        let self_prt = &boxed.a as *const String;
        unsafe {
            boxed.as_mut().get_unchecked_mut().b = self_prt;
        };

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

#[test]
fn test() {
    let mut test1 = Test::new("test1");

    let mut test2 = Test::new("test2");

    println!(
        "test1, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test1.as_ref().a(),
        test1.as_ref().b(),
        &test1.a as *const String,
        &test1.b
    );
    println!(
        "test2, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test2.as_ref().a(),
        test2.as_ref().b(),
        &test2.a as *const String,
        &test2.b
    );

    println!("================================================================================================");

    std::mem::swap(&mut test1, &mut test2);
    println!(
        "test1, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test1.as_ref().a(),
        test1.as_ref().b(),
        &test1.a as *const String,
        &test1.b
    );
    println!(
        "test2, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test2.as_ref().a(),
        test2.as_ref().b(),
        &test2.a as *const String,
        &test2.b
    );
}
