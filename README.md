# My Rust Playground

<img src="https://github.com/YassinEldeeb/Rust-playground/blob/main/Images/rustacean.svg" width="400" />

# My Rust's CheatSheat

## Rust's Ownership System

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

