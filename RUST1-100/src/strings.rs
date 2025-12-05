//syntax of strings in rust

fn main() {
    // When assigning a String in Rust:
    // On the stack, a variable holds a pointer to the heap (where the string data is stored),
    // along with the length and capacity of the string.
    // The actual string data ("shatadal samui") is stored on the heap.

    let name = String::from("shatadal samui");

    println!("{}", name);

    let mut name2 = String::from("Shatadal");

    // pushing to strings 
    name2.push_str(" Samui");

    println!("{}", name2);
}
