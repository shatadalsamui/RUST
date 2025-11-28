fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership gets transferred 
    //println!("{}", s1); // This line would cause a compile error because ownership has been moved.
}