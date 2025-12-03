fn main() {
    let mut vec = vec![12, 15, 67, 87, 88, 99, 07, 24];
    even_filter(&mut vec); // mutable borrow 
    println!("{:?}", vec);
}

fn even_filter(vec: &mut Vec<i32>) {
    vec.retain(|&x| x % 2 == 0);
}
