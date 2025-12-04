// .iter() -> iterate over an iterable object
// Use when you want immutable references to the elements
// and do not want to transfer ownership.

// .iter_mut() -> iterate and update values
// Use when you want mutable references to the elements
// and do not want to transfer ownership.

// .into_iter() ->
// Use when you want to move (take ownership of) the elements into the iterator
// and do not need to use the original collection afterwards.


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
