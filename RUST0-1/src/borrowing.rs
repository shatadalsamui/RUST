fn main() {
    let my_string = String::from("hello lets get rusty");
    // Pass a reference to 'my_string' to the function.
    // No ownership is transferred; 'takes_ownership' borrows 'my_string'.
    borrows(&my_string);
    borrows2(&my_string);
    println!("{}", my_string);
}
// Function takes a reference to a String, not ownership.
fn borrows(some_string: &String) {
    println!("2. {}", some_string);
}

//can be borrowed multiple times as its immuatble string 
fn borrows2(some_string: &String) {
    println!("1. {}", some_string);
}