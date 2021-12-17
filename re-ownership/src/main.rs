fn main() {
    // ---- Ownership rules ----
    // 1. Each variable in Rust has a variable that's called it's owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value is dropped.
    println!("Hello, world!");

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1; // Move (not shallow copy)

    println!("{}", s1);
}
