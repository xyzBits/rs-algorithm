#[test]
fn test_1() {
    let mut num = 5;

    let r1 = &num as *const i32;

    unsafe {
        println!("{}", *r1);
    }

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("{}", *r2);
    }
}

#[test]
fn test_2() {}
