fn main() {
    let s1 = String::from("hello world");
    let s2 = &s1;// passing the address of s1 to s2 
    println!("{}", s2);
    println!("{}", s1);
}
