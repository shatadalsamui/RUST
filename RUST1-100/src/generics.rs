fn main() {
    let bigger_number = largest(1, 2);
    let bigger_char = largest('a', 'b');

    println!("Bigger number is {}", bigger_number);

    println!("Bigger character is {}", bigger_char);
}

// The function 'largest' is generic over type T, which means it can accept any type (like i32, char, etc.)
// The syntax 'T: std::cmp::PartialOrd' is a trait bound, which restricts T to types that can be compared using >, <, etc.
// This is necessary because we use 'a > b' inside the function, so Rust must know that T supports these comparison operators.
// We define this function above 'main' so it can be used anywhere below, including in 'main'.
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    } else {
        return b;
    }
}