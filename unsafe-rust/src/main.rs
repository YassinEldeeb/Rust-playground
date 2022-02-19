use core::slice;

// Global(Static) Variables
static HELLO_GUYS: &str = "Hello, Guys!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("{}", HELLO_GUYS);

    add_to_count(10);
    unsafe {
        println!("{}", COUNTER);
    }

    // Raw Pointers
    let mut num = vec![1, 2, 3, 4, 5, 6];

    let r1 = &num as *const Vec<i32>;
    let r2 = &mut num as *mut Vec<i32>;

    unsafe {
        println!("Immutable Raw Pointer: {:?}", *r1);
        println!("mutable Raw Pointer: {:?}", *r2);
    }

    // Calling unsafe functions/methods
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // Creating a Safe Abstraction over Unsafe Code
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = split_at_mut(&mut v, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // using `extern` Functions to call external code (FFI)
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Exposing our own functions (ABI)
    #[no_mangle]
    extern "C" fn call_from_c() {
        println!("Just called a function from C!");
    }

    // Implementing an Unsafe Trait

    unsafe trait Foo {}
    unsafe impl Foo for i32 {}
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(len >= mid);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
