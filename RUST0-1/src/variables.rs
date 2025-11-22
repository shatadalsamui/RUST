pub fn explore_numbers() {
    let x: i32 = -1; //i8,i16,132,164,i128-integer
    println!("x: {}", x);

    let y: u32 = 1; //u8,u16,u32,u64,u128-unsigned integer
    println!("y: {}", y);

    let z: f32 = 10.321321; // f8,16,32,64,128 decimals
    println!("z: {}", z);

    //demonstrate_overflow();
    boolean();
    string();
}

// This function demonstrates integer overflow on an i8 by repeatedly adding 127.
// It compiles (Rust's compiler performs static checks like types and borrowing),
// but overflow is a runtime behavior:
pub fn demonstrate_overflow() {
    // By default, variables in Rust are immutable and cannot be changed or updated.
    // To allow changes, add 'mut' to make the variable mutable.
    let mut num: i8 = 124;
    for _ in 0..100 {
        num += 127;
    }

    println!("Number: {}", num);
}

//simple boolean in rust
pub fn boolean() {
    let is_male: bool = true;
    let is_above_18: bool = true;

    if is_male {
        println!("you are male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18{
        println!("you are a legal male");
    }
}

pub fn string(){
    let greeting = String::from("hello world"); //heap allocated growable string instance
    println!("{}", greeting );
}