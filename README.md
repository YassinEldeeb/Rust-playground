# My Rust Playground

<img src="https://github.com/YassinEldeeb/Rust-playground/blob/main/Images/rustacean.svg" width="400" />

# Trying to understand some concepts by teaching 👨‍🏫👇

## Rust's Ownership System

### Benifits
1. Prevents Data racists.
2. Prevents Dangling pointers.
3. Prevents the `double free` error.
4. Ensures performance when dealing with memory on the heap.

### Ownership Rules

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### - Data stored on the `Stack`
```rs
    let x = 5;
    let y = x;
```
1. `5` will be binded to `x`.
2. a copy is made from the value of the variable `x` to bind to the `y` variable.

> That's becuase numbers are one of the types that implement the `Copy` trait.
> 
> All data stored on the `stack` must have a known, fixed size at compile time.

### - Data stored on the `Heap`

```rs
    let s1 = String::from("hello");
    let s2 = s1;
```

This is called a move; `s1` pointer is invalidated and s2 copies the pointer of `s1` that's stored on the `stack`.

> `Deep copy` means to copy the actual data stored on the heap and store a pointer on the `stack` for it.
> 
> `Shallow Copy` means to only copy the pointer, length, and capacity without copying the actual data.

![Move a variable Diagram](https://doc.rust-lang.org/book/img/trpl04-04.svg)

## Why `s1` is invalidated?
### Why not a deep copy?
If Rust instead copied the heap data as well, the operation `s2 = s1` could be very expensive in terms of runtime performance if the data on the heap were large.

### Why not a shallow copy?

when a variable goes out of scope, Rust automatically calls the `drop` function and cleans up the `heap` memory for that variable. But the previous diagram shows both data pointers pointing to the same location. This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as a *`double free`* error and is one of the memory safety bugs. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, after the line `let s2 = s1`, Rust considers `s1` as no longer valid. Therefore, Rust doesn’t need to free anything when `s1` goes out of scope.

## Clone data on the Heap?

There’s a design choice that’s implied by this: Rust will never automatically create `deep` copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`.

> When you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.
```rs
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

### But why we didn't call `clone` here? but `x` is still valid and wasn't moved into `y`

```rs
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent `x` from being valid after we create the variable `y`. In other words, there’s no difference between deep and shallow copying here, so calling `clone` wouldn’t do anything different from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the `stack` like integers are. If a type implements the `Copy` trait, a variable is still valid after assignment to another variable. Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we’ll get a compile-time error.

### so What types implement the `Copy` trait?

#### ✨ As a general rule:
any group of simple scalar values can implement `Copy` and nothing that requires allocation or is some form of resource can implement `Copy`


- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

## References `&` (Borrowing)
A reference is like a pointer in that it’s an address we can follow to access data stored at that address that is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.

```rs
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
![](https://doc.rust-lang.org/book/img/trpl04-05.svg)

## Mutable References `&mut`

```rs
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. This code that attempts to create two mutable references to `s` will fail:

⚠ Won't compile
```rs
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent *`data races`* at compile time. A `data race` is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rs
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```rs
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
```

Whew! We also cannot have a mutable reference while we have an immutable one to the same value. Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

### Remember that reference's scope is different than variable scope

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references, the `println!`, occurs before the mutable reference is introduced:

```rs
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

> The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short)

## Dangling References
In languages with pointers, it’s easy to erroneously create a dangling pointer--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:
```rs
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid `String`. That’s no good! Rust won’t let us do this.

The solution here is to return the String directly:

```rs
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
This works without any problems. Ownership is moved out, and nothing is deallocated.


### The Rules of References
Let’s recap what we’ve discussed about references:
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.


## The Slice Type
*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

```rs
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```
hello is a reference to a portion of the String, specified in the extra [0..5] bit. We create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.

![](https://doc.rust-lang.org/book/img/trpl04-06.svg)

With Rust’s .. range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:


```rs
let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the String, you can drop the trailing number.

```rs
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these are equal:

```rs
let slice = &s[..];
```

### Other Slices

Just as we might want to refer to a part of a string, we might want to refer to part of an array. We’d do so like this:
```rs
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

This slice has the type `&[i32]`. It works the same way as string slices do, by storing a reference to the first element and a length. You’ll use this kind of slice for all sorts of other collections.

## Rust's Module System

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

### Summary
Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.
