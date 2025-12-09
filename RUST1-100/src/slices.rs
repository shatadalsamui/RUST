fn main() {
    let name = String::from("hello world"); 

    let ans = first_word(&name); // pass a reference to the string

    println!("ans is {} ", ans);
}

// This function takes a reference to a String and returns a string slice (&str).
// No new memory is allocated; the returned slice borrows from the original string.
fn first_word(str: &String) -> &str {
    let mut space_here = 0;

    // Iterate through the characters until the first space is found
    for value in str.chars() {
        if value == ' ' {
            break;
        }
        space_here += 1;
    }

    // Return a slice of the string from the start up to the first space
    &str[0..space_here]
}