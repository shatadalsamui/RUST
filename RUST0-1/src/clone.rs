fn main() {
    // s1 is a String stored on the stack, with its data on the heap.
    // s2 = s1.clone() creates a new String on the stack and also makes a new copy of the data on the heap.
    // Both s1 and s2 are in the same stack frame, but each has its own separate heap data.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1); // Compiles now
}