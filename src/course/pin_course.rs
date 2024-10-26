//!
//! test1, a: test1, b: test1, point a: 0x93cc2feb90, point b: 0x93cc2feb90
//! test2, a: test2, b: test2, point a: 0x93cc2febb0, point b: 0x93cc2febb0
//! ================================================================================================
//! test1, a: test2, b: test1, point a: 0x93cc2feb90, point b: 0x93cc2febb0
//! test2, a: test1, b: test2, point a: 0x93cc2febb0, point b: 0x93cc2feb90
//! 问题：
//!     1。 成员 a 交易的是值内容，成员a在内存地址没有发生变化，只是内存里面的数据发生了交换
//!     2。 成员 b 是指针类型，所以交换的是地址
//!     3。 交易之后  test1.b 指向 test2.a test2.b 指向 test1.a
//!     如果  test2 析构了，这个时候 test1.b 就是悬挂指针了，野指针
//!
//! !xxx 和 xxx!
//! !xxx 表示未实现，例如 !UnPin !Send !Sync 都是表示未实现某个 trait
//! xxx！ 是宏，如 println!

use std::pin::Pin;

#[derive(Debug)]
struct Test {
    a: String,
    // String 的裸指针
    b: *const String,
}

impl Test {
    fn new(text: &str) -> Test {
        Test {
            a: String::from(text),
            b: std::ptr::null(),
        }
    }

    // 成员 b 是一个指针，指向了成员 a 的地址，可以理解成 a 引用了 a
    // 因此  b a 的内存地址是一样的，这种内部成员之间的相互引用，称之为自引用类型
    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(
            !self.b.is_null(),
            "Test::b called with out Test::init being called first"
        );
        unsafe { &*(self.b) }
    }
}

#[test]
fn test_1() {
    let mut test1 = Test::new("test1");
    test1.init();

    let mut test2 = Test::new("test2");
    test2.init();

    println!(
        "test1, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test1.a(),
        test1.b(),
        &test1.a as *const String,
        &test1.b
    );
    println!(
        "test2, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test2.a(),
        test2.b(),
        &test2.a as *const String,
        &test2.b
    );

    println!("================================================================================================");

    std::mem::swap(&mut test1, &mut test2);
    println!(
        "test1, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test1.a(),
        test1.b(),
        &test1.a as *const String,
        &test1.b
    );
    println!(
        "test2, a: {}, b: {}, point a: {:?}, point b: {:?}",
        test2.a(),
        test2.b(),
        &test2.a as *const String,
        &test2.b
    );
}

#[test]
fn test_2() {
    let mut test1 = Test::new("test1");
    let mut test2 = Test::new("test2");

    let test1_pin = unsafe { Pin::new_unchecked(&mut test1) };
    drop(test1_pin);

    std::mem::swap(&mut test1, &mut test2);
}
