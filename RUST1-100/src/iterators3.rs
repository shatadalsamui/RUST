fn main() {
    let mut nums = vec![1, 2, 3];
    for value in nums.iter_mut() {
        *value += 10;

    }
    println!("{:?}", nums);
}
