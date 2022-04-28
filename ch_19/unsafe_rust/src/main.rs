use std::slice;

static mut COUNTER: u32 = 0;

fn main() {
    // Using raw pointers
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calling an unsafe function
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // split_as_mut is implemented using unsafe Rust
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);

    // Using extern functions to call external code
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Modifying a mutable static variable
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Implementing an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}