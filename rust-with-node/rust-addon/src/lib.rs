#[no_mangle]
pub extern "C" fn add_two_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
