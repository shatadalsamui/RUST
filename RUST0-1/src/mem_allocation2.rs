
fn main() {
    let x = 1; // 1. stack allocated in stack frame 1
    let y = 3; // 2. stack allocated in stack frame 1

    println!("{}", sum(x, y)); // 6. function called and sum gets printed
    println!("Hello World!"); // 7. after printing hello world, stack frame 1 is cleaned up when main ends
}

fn sum(a: i32, b: i32) -> i32 { // 3. new stack frame 2, a and b allocated in stack frame 2
    // Note: stack frame 2 (for sum) is created inside stack frame 1 (main)
    let c = a + b; // 4. c is allocated in stack frame 2
    return c; // 5. returns c, stack frame 2 is cleaned up
}
