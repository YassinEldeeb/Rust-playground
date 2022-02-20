fn main() {
    let v = vec2![1, 2, 3, 4, 5];
    println!("{:?}", v);
}

#[macro_export]
macro_rules! vec2 {
    ($($x:expr),*) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}
