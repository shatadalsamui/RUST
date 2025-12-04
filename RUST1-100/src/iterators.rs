fn main() {
    let nums = vec![1, 2, 3];

    let iter_nums = nums.iter();

    for value in iter_nums {
        println!("{}", value);
    }

    let nums2 = vec![1, 2, 3];

    let iter_nums2 = nums2.iter();

    for (i, value) in iter_nums2.enumerate() {
        println!("Index: {}, Value: {}", i, value);
    }
}
