extern "C" {
    fn abs(input: i32) -> i32;
}


fn main() {
    // 19.1 unsafe code

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // *r1 = 6;
        println!("r1 is: {}", *r1);

        *r2 = 7;
        println!("r2 is: {}", *r2);
        println!("r1 is: {}", *r1);
    }

    println!("num is: {}", num);

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
