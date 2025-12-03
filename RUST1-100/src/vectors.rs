fn main(){
    // Create a new, empty vector to store i32 integers.
    // Vectors are growable arrays that store values of the same type in contiguous memory.
    let mut vec = Vec::new(); 
    
    vec.push(1324);
    vec.push(2324);
    vec.push(4332);
    vec.push(1342);
    vec.push(1432);

    println!("{:?}",vec);
}


