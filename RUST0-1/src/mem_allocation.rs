fn main() {
    stack_fn();
    heap_fn();
    update_string();
}

// When this function is called, the 3 variables are stack-allocated (default 32-bit size each).
// After the print statement and when the function returns, all 3 are deallocated from the stack.
fn stack_fn() {
    let a = 1;
    let b = 3;
    let c = a + b;
    println!("sum : {}", c);
}

// When this function is called, each String variable (s1, s2, combined) is represented on the stack
// as a struct containing a pointer, length, and capacity. The pointer points to an address in the heap
// where the actual string data is stored as a contiguous sequence of bytes (1 byte per ASCII character).
// The combined String is also allocated on the heap in the same way. When the function ends, all heap memory
// used by these Strings is automatically deallocated.
fn heap_fn() {
    let s1 = String::from("Hello ");
    let s2 = String::from("World");
    let combined = format!("{}{}", s1, s2);
    println!("combined string : {}", combined);
}

// Demonstrates mutating a heap-allocated String by appending text and showing the change before and after.
fn update_string() {
    let mut s = String::from("Initial String");
    println!("Before update: {}", s);
    s.push_str(" and some additional text");
    println!("After update: {}", s);
}
