fn main() {
    let mut s1 = String::from("hello lets get rusty");//mutable string

    update_string(&mut s1);// pass the mutable reference of s1 to update_string function
    println!("{}", s1);
}

// Function that takes a mutable reference to a String.
fn update_string(some_string: &mut String) {
    some_string.push_str(" and develop something");
}
