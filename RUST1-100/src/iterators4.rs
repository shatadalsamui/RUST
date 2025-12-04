fn main() {
    let mut nums = vec![1, 2, 3];
    
    //Gives mutable reference to each element in the vector 
    let iter = nums.iter_mut();

    for value in iter {
        *value = *value + 1;
    }

    println!("{:?}",nums);
}
