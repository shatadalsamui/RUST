fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let odds: Vec<_> = nums.into_iter().filter(|x| x % 2 != 0).collect();

    println!("{:?}", odds); // [1, 3, 5]
}